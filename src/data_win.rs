//! Minimal data.win (GameMaker "FORM" IFF container) writer.
//!
//! # Design
//! Each GameMaker chunk is represented as a plain Rust struct whose fields
//! map directly to the on-disk layout.  Assemble them into a [`DataWin`] and
//! call [`DataWin::build`] to obtain the complete raw file bytes.
//!
//! For the common single-script case, [`build_data_win`] / [`write_data_win`]
//! provide a one-call shorthand.
//!
//! # Targets
//! This targets the "wadVersion 13" layout (roughly GameMaker: Studio 1.4),
//! where CODE / VARI / FUNC use the flat, header-free encoding.
//!
//! # Offset conventions (the easiest thing to get backwards)
//!   1. Every OTHER chunk's "string pointer" field stores the ABSOLUTE FILE
//!      OFFSET of the string's first character byte.
//!   2. STRG's own internal pointer table stores the absolute file offset
//!      of the 4-byte length PREFIX that precedes those characters
//!      (i.e. char_offset - 4).

use std::io;
use std::path::Path;

use crate::Program;

const WAD_VERSION: u8 = 13;

// ================================================================
// Public chunk structs
// ================================================================

/// GEN8 chunk – game metadata header.
pub struct Gen8 {
    pub is_debugger_disabled: bool,
    pub file_name: String,
    pub config: String,
    pub last_obj: u32,
    pub last_tile: u32,
    pub game_id: u32,
    pub direct_play_guid: [u8; 16],
    pub name: String,
    pub major: u32,
    pub minor: u32,
    pub release: u32,
    pub build_number: u32,
    pub default_window_width: u32,
    pub default_window_height: u32,
    /// GEN8 info bitflags.
    pub info: u32,
    pub license_crc32: u32,
    pub license_md5: [u8; 16],
    pub timestamp: u64,
    pub display_name: String,
    pub active_targets: u64,
    pub function_classifications: u64,
    pub steam_app_id: i32,
    /// Room indices in load order; must correspond to rooms in the ROOM chunk
    /// by index.
    pub room_order: Vec<i32>,
}

impl Default for Gen8 {
    fn default() -> Self {
        Gen8 {
            is_debugger_disabled: false,
            file_name: "mygame".to_string(),
            config: "Configs\\Default".to_string(),
            last_obj: 0,
            last_tile: 0,
            game_id: 0x1234_5678,
            direct_play_guid: [0; 16],
            name: "MyGame".to_string(),
            major: 1,
            minor: 4,
            release: 9999,
            build_number: 0,
            default_window_width: 640,
            default_window_height: 480,
            info: 0,
            license_crc32: 0,
            license_md5: [0; 16],
            timestamp: 0,
            display_name: "My Game".to_string(),
            active_targets: 0,
            function_classifications: 0,
            steam_app_id: 0,
            room_order: vec![0],
        }
    }
}

impl Gen8 {
    fn serialize(&self, pool: &mut StringPool) -> ChunkBuilder {
        let mut c = ChunkBuilder::new("GEN8");

        c.u8(self.is_debugger_disabled as u8);
        c.u8(WAD_VERSION);
        c.zero_bytes(2); // padding

        c.str_ref(pool, &self.file_name);
        c.str_ref(pool, &self.config);
        c.u32(self.last_obj);
        c.u32(self.last_tile);
        c.u32(self.game_id);
        c.bytes(&self.direct_play_guid);

        c.str_ref(pool, &self.name);
        c.u32(self.major);
        c.u32(self.minor);
        c.u32(self.release);
        c.u32(self.build_number);
        c.u32(self.default_window_width);
        c.u32(self.default_window_height);
        c.u32(self.info);
        c.u32(self.license_crc32);
        c.bytes(&self.license_md5);

        // wadVersion > 12 -> full tail
        c.u64(self.timestamp);
        c.str_ref(pool, &self.display_name);
        c.u64(self.active_targets);
        c.u64(self.function_classifications);
        c.i32(self.steam_app_id);
        // wadVersion < 14 -> no debuggerPort field

        c.u32(self.room_order.len() as u32);
        for &idx in &self.room_order {
            c.i32(idx);
        }
        // major == 1 -> no GMS2 random-seed / FPS / GUID tail

        c
    }
}

/// OPTN chunk - engine / display options.
///
/// Always emitted with the "new" bitflag layout
/// (`shaderExtensionFlag = 0x8000_0000`, `shaderExtVersion = 1`).
pub struct Optn {
    /// Info bitflags; e.g. `0x10` = ShowCursor.
    pub info: u64,
    pub scale: i32,
    pub window_color: u32,
    pub color_depth: u32,
    pub resolution: u32,
    pub frequency: u32,
    pub vertex_sync: u32,
    pub priority: u32,
    pub back_image: u32,
    pub front_image: u32,
    pub load_image: u32,
    pub load_alpha: u32,
}

impl Default for Optn {
    fn default() -> Self {
        Optn {
            info: 0x10,
            scale: 0,
            window_color: 0,
            color_depth: 32,
            resolution: 0,
            frequency: 60,
            vertex_sync: 1,
            priority: 0,
            back_image: 0,
            front_image: 0,
            load_image: 0,
            load_alpha: 255,
        }
    }
}

impl Optn {
    fn serialize(&self) -> ChunkBuilder {
        let mut c = ChunkBuilder::new("OPTN");

        c.u32(0x8000_0000); // shaderExtensionFlag - selects new bitflag layout
        c.i32(1); // shaderExtVersion

        c.u64(self.info);
        c.i32(self.scale);
        c.u32(self.window_color);
        c.u32(self.color_depth);
        c.u32(self.resolution);
        c.u32(self.frequency);
        c.u32(self.vertex_sync);
        c.u32(self.priority);
        c.u32(self.back_image);
        c.u32(self.front_image);
        c.u32(self.load_image);
        c.u32(self.load_alpha);

        c.u32(0); // constantCount (wadVersion > 8 -> Constants list present, but empty)

        c
    }
}

/// A single room entry for the ROOM chunk.
pub struct Room {
    pub name: String,
    pub caption: String,
    pub width: u32,
    pub height: u32,
    pub speed: u32,
    pub persistent: bool,
    pub background_color: u32,
    pub draw_background_color: bool,
    /// Index into the CODE chunk whose bytecode runs when this room loads,
    /// or `-1` for none.
    pub creation_code_id: i32,
    pub flags: u32,
    pub world: bool,
    pub top: u32,
    pub left: u32,
    pub right: u32,
    pub bottom: u32,
    pub gravity_x: f32,
    pub gravity_y: f32,
    pub meters_per_pixel: f32,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            name: "room0".to_string(),
            caption: String::new(),
            width: 640,
            height: 480,
            speed: 30,
            persistent: false,
            background_color: 0x00FF_FFFF,
            draw_background_color: true,
            creation_code_id: -1,
            flags: 0,
            world: false,
            top: 0,
            left: 0,
            right: 640,
            bottom: 480,
            gravity_x: 0.0,
            gravity_y: 10.0,
            meters_per_pixel: 0.1,
        }
    }
}

/// A single compiled script entry for the CODE chunk.
///
/// `name` follows the GameMaker convention (`gml_Script_<name>`,
/// `gml_Object_<obj>_<event>`, `gml_RoomCC_<room>`, ...), though any unique
/// label works for runners that do not do name-based lookups.
#[derive(Clone)]
pub struct CodeEntry {
    pub name: String,
    pub bytecode: Vec<u8>,
}

/// Top-level container for all chunk data.
///
/// Populate the fields and call [`DataWin::build`] to get the raw bytes.
///
/// rust,ignore
/// let dw = DataWin {
///     gen8: Gen8 { display_name: "My Cool Game".to_string(), ..Gen8::default() },
///     rooms: vec![Room { creation_code_id: 0, ..Room::default() }],
///     code: vec![CodeEntry { name: "gml_RoomCC_room0".to_string(), bytecode }],
///     ..DataWin::default()
/// };
/// std::fs::write("data.win", dw.build())?;
pub struct DataWin {
    pub gen8: Gen8,
    pub optn: Optn,
    /// Rooms listed in ROOM-chunk order; `gen8.room_order` references these
    /// by index.
    pub rooms: Vec<Room>,
    /// Compiled scripts for the CODE chunk.
    pub code: Vec<CodeEntry>,
    /// Variable names referenced by code; written to the VARI chunk.
    pub variables: Vec<String>,
    /// Function names referenced by code; written to the FUNC chunk.
    pub functions: Vec<String>,
}

impl Default for DataWin {
    fn default() -> Self {
        DataWin {
            gen8: Gen8::default(),
            optn: Optn::default(),
            rooms: vec![Room::default()],
            code: Vec::new(),
            variables: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl DataWin {
    /// Serialise all chunks into a complete `data.win` byte stream.
    pub fn build(&self) -> Vec<u8> {
        let mut pool = StringPool::new();

        // Chunk order matches the conventional GameMaker data.win layout.
        let mut chunks: Vec<ChunkBuilder> = vec![
            self.gen8.serialize(&mut pool),
            self.optn.serialize(),
            ChunkBuilder::empty_list("EXTN"),
            ChunkBuilder::empty_list("SOND"),
            ChunkBuilder::empty_list("AGRP"),
            ChunkBuilder::empty_list("SPRT"),
            ChunkBuilder::empty_list("BGND"),
            ChunkBuilder::empty_list("PATH"),
            ChunkBuilder::empty_list("SCPT"),
            ChunkBuilder::empty_list("GLOB"),
            ChunkBuilder::empty_list("SHDR"),
            ChunkBuilder::empty_list("FONT"),
            ChunkBuilder::empty_list("TMLN"),
            ChunkBuilder::empty_list("OBJT"),
            serialize_rooms(&self.rooms, &mut pool),
            ChunkBuilder::empty_list("TPAG"),
            serialize_code(&self.code, &mut pool),
            serialize_vari(&self.variables, &mut pool),
            serialize_func(&self.functions, &mut pool),
            ChunkBuilder::new("STRG"), // placeholder; rebuilt once its base offset is known
            ChunkBuilder::empty_list("TXTR"),
            ChunkBuilder::empty_list("AUDO"),
        ];

        layout_and_patch(&mut chunks, &pool)
    }
}

// ================================================================
// Convenience entry points
// ================================================================

/// Build a complete `data.win` from a single compiled script and its
/// variable/function references. The script is wired up as `room0`'s
/// creation code so a standard runner executes it on room load.
pub fn build_data_win(code_name: &str, program: Program) -> Vec<u8> {
    DataWin {
        rooms: vec![Room {
            creation_code_id: 0,
            ..Room::default()
        }],
        code: vec![CodeEntry {
            name: code_name.to_string(),
            bytecode: program.bytecode.data,
        }],
        variables: program.variables.into_iter().map(|v| v.name).collect(),
        functions: program.functions.into_iter().map(|f| f.name).collect(),
        ..DataWin::default()
    }
    .build()
}

/// Like [`build_data_win`], but accepts multiple compiled scripts.
/// Only `code[0]` is wired to `room0`'s creation code; the rest ride along
/// in the CODE chunk for runners that look scripts up by name or index.
pub fn build_data_win_multi(
    code: &[CodeEntry],
    variables: &[String],
    functions: &[String],
) -> Vec<u8> {
    DataWin {
        rooms: vec![Room {
            creation_code_id: if code.is_empty() { -1 } else { 0 },
            ..Room::default()
        }],
        code: code.to_vec(),
        variables: variables.to_vec(),
        functions: functions.to_vec(),
        ..DataWin::default()
    }
    .build()
}

/// Convenience wrapper: build and write straight to disk.
pub fn write_data_win(path: impl AsRef<Path>, code_name: &str, program: Program) -> io::Result<()> {
    std::fs::write(path, build_data_win(code_name, program))
}

// ================================================================
// Private chunk serializers
// ================================================================

fn serialize_rooms(rooms: &[Room], pool: &mut StringPool) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("ROOM");

    c.u32(rooms.len() as u32);
    let ptr_positions: Vec<usize> = (0..rooms.len())
        .map(|_| c.local_ref_placeholder())
        .collect();

    for (room, ptr_pos) in rooms.iter().zip(ptr_positions) {
        let room_start = c.pos();
        c.local_ref_set(ptr_pos, room_start);

        c.str_ref(pool, &room.name);
        c.str_ref(pool, &room.caption);
        c.u32(room.width);
        c.u32(room.height);
        c.u32(room.speed);
        c.bool32(room.persistent);
        c.u32(room.background_color);
        c.bool32(room.draw_background_color);
        c.i32(room.creation_code_id);
        c.u32(room.flags);

        let bg_off = c.local_ref_placeholder();
        let view_off = c.local_ref_placeholder();
        let obj_off = c.local_ref_placeholder();
        let tile_off = c.local_ref_placeholder();

        c.bool32(room.world);
        c.u32(room.top);
        c.u32(room.left);
        c.u32(room.right);
        c.u32(room.bottom);
        c.f32(room.gravity_x);
        c.f32(room.gravity_y);
        c.f32(room.meters_per_pixel);

        let bg_list = c.pos();
        c.u32(0); // 0 backgrounds
        let view_list = c.pos();
        c.u32(0); // 0 views
        let obj_list = c.pos();
        c.u32(0); // 0 objects
        let tile_list = c.pos();
        c.u32(0); // 0 tiles

        c.local_ref_set(bg_off, bg_list);
        c.local_ref_set(view_off, view_list);
        c.local_ref_set(obj_off, obj_list);
        c.local_ref_set(tile_off, tile_list);
    }

    c
}

/// CODE chunk: PointerList of compiled scripts (old / wadVersion <= 14 format).
/// Each entry is `name, length, <raw bytes>` - no locals/arguments header.
fn serialize_code(code: &[CodeEntry], pool: &mut StringPool) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("CODE");

    c.u32(code.len() as u32);
    let ptr_positions: Vec<usize> = (0..code.len()).map(|_| c.local_ref_placeholder()).collect();

    for (entry, ptr_pos) in code.iter().zip(ptr_positions) {
        let entry_start = c.pos();
        c.local_ref_set(ptr_pos, entry_start);

        c.str_ref(pool, &entry.name);
        c.u32(entry.bytecode.len() as u32);
        c.bytes(&entry.bytecode);
    }

    c
}

/// VARI chunk (wadVersion <= 14 "old format"): 12 bytes per variable, no
/// header. `occurrences = 0` / `firstAddress = -1` signals that the runtime
/// patch chain should not be walked (gmlc bakes variable IDs at compile time).
fn serialize_vari(variables: &[String], pool: &mut StringPool) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("VARI");
    for name in variables {
        c.str_ref(pool, name);
        c.u32(0); // occurrences
        c.i32(-1); // firstAddress sentinel (no patch chain)
    }
    c
}

/// FUNC chunk: same layout as VARI.
fn serialize_func(functions: &[String], pool: &mut StringPool) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("FUNC");
    for name in functions {
        c.str_ref(pool, name);
        c.u32(0); // occurrences
        c.i32(-1); // firstAddress sentinel (no patch chain)
    }
    c
}

// ================================================================
// Layout and patch pass
// ================================================================

/// Compute the final file layout, build the real STRG chunk, back-fill all
/// placeholder offsets, then concatenate everything into the finished bytes.
fn layout_and_patch(chunks: &mut Vec<ChunkBuilder>, pool: &StringPool) -> Vec<u8> {
    let strg_idx = chunks.iter().position(|c| &c.name == b"STRG").unwrap();

    // Stage 1: lay out every chunk before STRG (sizes don't depend on where
    // strings end up, only on the fixed 4-byte placeholders already written).
    let mut base_offsets = vec![0usize; chunks.len()];
    let mut cursor = 8usize; // past "FORM" + u32 total-length
    for i in 0..strg_idx {
        base_offsets[i] = cursor + 8; // skip this chunk's own 8-byte header
        cursor += 8 + chunks[i].data.len();
    }
    base_offsets[strg_idx] = cursor + 8;

    // Stage 2: build the real STRG chunk now that its base offset is known,
    // computing each string's length-prefix offset (for STRG's own pointer
    // table) and character offset (for every other chunk's string references).
    let strg_base = base_offsets[strg_idx];
    let mut strg = ChunkBuilder::new("STRG");
    strg.u32(pool.strings.len() as u32);
    let ptr_table_pos = strg.pos();
    for _ in &pool.strings {
        strg.u32(0); // placeholder pointer-table entries, filled below
    }
    let mut char_offsets = Vec::with_capacity(pool.strings.len());
    for (i, s) in pool.strings.iter().enumerate() {
        let len_prefix_rel = strg.pos();
        strg.u32(s.len() as u32);
        strg.bytes(s.as_bytes());
        strg.u8(0); // NUL terminator

        let abs_len_prefix = strg_base + len_prefix_rel;
        strg.set_u32(ptr_table_pos + i * 4, abs_len_prefix as u32);
        char_offsets.push((abs_len_prefix + 4) as u32);
    }
    chunks[strg_idx] = strg;

    // Stage 3: lay out everything after STRG using its real size.
    cursor = strg_base + chunks[strg_idx].data.len();
    for i in (strg_idx + 1)..chunks.len() {
        base_offsets[i] = cursor + 8;
        cursor += 8 + chunks[i].data.len();
    }
    let total_file_size = cursor;

    // Stage 4: back-fill every placeholder across all chunks.
    for (i, c) in chunks.iter_mut().enumerate() {
        let patches = c.patches.clone();
        for p in patches {
            match p {
                Patch::Str(pos, sid) => c.set_u32(pos, char_offsets[sid]),
                Patch::Local(pos, target_rel) => {
                    c.set_u32(pos, (base_offsets[i] + target_rel) as u32)
                }
            }
        }
    }

    // Stage 5: concatenate into the final file.
    let mut out = Vec::with_capacity(total_file_size);
    out.extend_from_slice(b"FORM");
    out.extend_from_slice(&((total_file_size - 8) as u32).to_le_bytes());
    for c in chunks.iter() {
        out.extend_from_slice(&c.name);
        out.extend_from_slice(&(c.data.len() as u32).to_le_bytes());
        out.extend_from_slice(&c.data);
    }
    debug_assert_eq!(out.len(), total_file_size);
    out
}

// ================================================================
// Low-level byte builder (private)
// ================================================================

/// One placeholder that must be back-filled once the final layout is known.
#[derive(Clone, Copy)]
enum Patch {
    /// `(byte offset in chunk data, string id)` -> absolute file offset of
    /// the string's character bytes.
    Str(usize, usize),
    /// `(byte offset in chunk data, target offset relative to this chunk's
    /// data start)` -> absolute file offset (`chunk_base + target`).
    Local(usize, usize),
}

struct ChunkBuilder {
    name: [u8; 4],
    data: Vec<u8>,
    patches: Vec<Patch>,
}

impl ChunkBuilder {
    fn new(name: &str) -> Self {
        let bytes = name.as_bytes();
        assert_eq!(bytes.len(), 4, "chunk names are always 4 bytes");
        let mut n = [0u8; 4];
        n.copy_from_slice(bytes);
        ChunkBuilder {
            name: n,
            data: Vec::new(),
            patches: Vec::new(),
        }
    }

    /// A chunk with nothing but a single `0` count - the shape every
    /// list-bearing-but-empty chunk takes.
    fn empty_list(name: &str) -> Self {
        let mut c = Self::new(name);
        c.u32(0);
        c
    }

    fn pos(&self) -> usize {
        self.data.len()
    }

    fn u8(&mut self, v: u8) {
        self.data.push(v);
    }
    fn u32(&mut self, v: u32) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }
    fn i32(&mut self, v: i32) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }
    fn u64(&mut self, v: u64) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }
    fn f32(&mut self, v: f32) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }
    fn bool32(&mut self, v: bool) {
        self.u32(if v { 1 } else { 0 });
    }
    fn bytes(&mut self, b: &[u8]) {
        self.data.extend_from_slice(b);
    }
    fn zero_bytes(&mut self, n: usize) {
        self.data.extend(std::iter::repeat(0u8).take(n));
    }

    fn set_u32(&mut self, pos: usize, v: u32) {
        self.data[pos..pos + 4].copy_from_slice(&v.to_le_bytes());
    }

    /// Write 4 zero bytes and record a [`Patch::Str`] to be resolved once
    /// the STRG chunk is finalised.
    fn str_ref(&mut self, pool: &mut StringPool, s: &str) {
        let id = pool.intern(s);
        let pos = self.pos();
        self.u32(0);
        self.patches.push(Patch::Str(pos, id));
    }

    /// Write 4 zero bytes and return the reserved position so the caller
    /// can later call [`Self::local_ref_set`] once the target offset is known.
    fn local_ref_placeholder(&mut self) -> usize {
        let pos = self.pos();
        self.u32(0);
        pos
    }

    /// Record a [`Patch::Local`] linking `placeholder_pos` (previously
    /// returned by [`Self::local_ref_placeholder`]) to `target_rel` (an
    /// offset relative to this chunk's data start).
    fn local_ref_set(&mut self, placeholder_pos: usize, target_rel: usize) {
        self.patches.push(Patch::Local(placeholder_pos, target_rel));
    }
}

struct StringPool {
    strings: Vec<String>,
}

impl StringPool {
    fn new() -> Self {
        StringPool {
            strings: Vec::new(),
        }
    }

    fn intern(&mut self, s: &str) -> usize {
        if let Some(id) = self.strings.iter().position(|x| x == s) {
            return id;
        }
        self.strings.push(s.to_string());
        self.strings.len() - 1
    }
}

// ================================================================
// Tests
// ================================================================

#[cfg(test)]
mod tests {
    use crate::{bytecode::Bytecode, resolver::Variable};

    use super::*;

    fn read_u32(data: &[u8], off: usize) -> u32 {
        u32::from_le_bytes(data[off..off + 4].try_into().unwrap())
    }

    fn find_chunk(data: &[u8], tag: &[u8; 4]) -> (usize, usize) {
        let mut pos = 8usize;
        while pos < data.len() {
            let name = &data[pos..pos + 4];
            let len = read_u32(data, pos + 4) as usize;
            let start = pos + 8;
            if name == tag {
                return (start, len);
            }
            pos = start + len;
        }
        panic!("chunk {:?} not found", std::str::from_utf8(tag));
    }

    fn read_cstr(data: &[u8], off: usize) -> String {
        let end = data[off..].iter().position(|&b| b == 0).unwrap() + off;
        String::from_utf8(data[off..end].to_vec()).unwrap()
    }

    #[test]
    fn round_trips_code_and_variables() {
        let bytecode = vec![0xDEu8, 0xAD, 0xBE, 0xEF, 0x01, 0x02];
        let variables = vec!["x".to_string(), "y".to_string(), "hp".to_string()];

        let out = build_data_win(
            "gml_RoomCC_room0",
            Program {
                bytecode: Bytecode {
                    data: bytecode.clone(),
                },
                variables: variables
                    .iter()
                    .map(|name| Variable {
                        name: name.clone(),
                        var_ref: 0,
                    })
                    .collect(),
                functions: Vec::new(),
            },
        );

        // FORM header is self-consistent.
        assert_eq!(&out[0..4], b"FORM");
        assert_eq!(read_u32(&out, 4) as usize, out.len() - 8);

        // CODE chunk contains our exact bytecode, addressable via its
        // pointer table.
        let (code_start, _) = find_chunk(&out, b"CODE");
        let count = read_u32(&out, code_start);
        assert_eq!(count, 1);
        let entry_ptr = read_u32(&out, code_start + 4) as usize;
        // name (strptr) then length then raw bytes
        let length = read_u32(&out, entry_ptr + 4) as usize;
        assert_eq!(length, bytecode.len());
        let body = &out[entry_ptr + 8..entry_ptr + 8 + length];
        assert_eq!(body, &bytecode[..]);

        // VARI chunk lists all three names, 12 bytes each, no header.
        let (vari_start, vari_len) = find_chunk(&out, b"VARI");
        assert_eq!(vari_len, variables.len() * 12);
        for (i, name) in variables.iter().enumerate() {
            let entry = vari_start + i * 12;
            let name_off = read_u32(&out, entry) as usize;
            assert_eq!(read_cstr(&out, name_off), *name);
        }

        // Room's creationCodeId points at our one CODE entry (index 0).
        let (room_start, _) = find_chunk(&out, b"ROOM");
        let room0_ptr = read_u32(&out, room_start + 4) as usize;
        // name(4) caption(4) width(4) height(4) speed(4) persistent(4)
        // bgcolor(4) drawbg(4) = 32 bytes in, then creationCodeId(i32).
        let creation_code_id =
            i32::from_le_bytes(out[room0_ptr + 32..room0_ptr + 36].try_into().unwrap());
        assert_eq!(creation_code_id, 0);
    }

    #[test]
    fn empty_code_disables_room_creation_code() {
        let out = build_data_win_multi(
            &[],
            &["x".to_string(), "y".to_string()],
            &["foo".to_string()],
        );
        let (room_start, _) = find_chunk(&out, b"ROOM");
        let room0_ptr = read_u32(&out, room_start + 4) as usize;
        let creation_code_id =
            i32::from_le_bytes(out[room0_ptr + 32..room0_ptr + 36].try_into().unwrap());
        assert_eq!(creation_code_id, -1);
    }
}
