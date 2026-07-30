//! Minimal data.win (GameMaker "FORM" IFF container) writer, built to be
//! dropped into `gmlc` as a plain module (`mod datawin;`).
//!
//! This targets the "wadVersion 13" layout (roughly GameMaker: Studio 1.4),
//! which is the simplest fully-specified branch in the reference parser at
//! https://github.com/ButterscotchRunner/Butterscotch/blob/main/src/data_win.c
//! and — not coincidentally — the branch where CODE/VARI/FUNC use the flat,
//! header-free encoding, which keeps this writer small.
//!
//! # Entry point
//! Call [`build_data_win`] with your compiled bytecode and the variable
//! names it references; you get back the complete file bytes to write
//! wherever you like. [`CodeEntry`] exists so you can later hand this
//! multiple scripts instead of one - see the "Extending" notes at the
//! bottom of this file.
//!
//! The compiled bytecode is wired up as the ROOM's `creationCodeId`, so a
//! runner that implements standard GameMaker room-load semantics will
//! execute it automatically when the (single, otherwise-empty) room loads.
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

// ---- wire-format wad version we are emitting (see module docs) ----
const WAD_VERSION: u8 = 13;

/// A single compiled script, ready to be embedded in the CODE chunk.
///
/// `name` is just a label (GameMaker convention is `gml_Script_<name>` /
/// `gml_Object_<obj>_<event>` / `gml_RoomCC_<room>`, but a runner that
/// doesn't do name-based lookups can use anything unique).
///
/// `bytecode` is the raw compiled instruction stream from `gmlc`, embedded
/// byte-for-byte with no framing beyond the length prefix GameMaker itself
/// expects.
pub struct CodeEntry {
    pub name: String,
    pub bytecode: Vec<u8>,
}

/// Build a complete data.win file containing one compiled script (wired up
/// as the room's creation code) and the variable names it references.
///
/// `code_name` becomes the CODE entry's name. `bytecode` is gmlc's
/// compiled output for that script. `variables` is the list of variable
/// names gmlc collected while compiling it (order doesn't matter to this
/// writer; each becomes one VARI entry).
///
/// Returns the raw file bytes - write them wherever you like, or use
/// [`write_data_win`] as a shortcut.
pub fn build_data_win(code_name: &str, program: Program) -> Vec<u8> {
    build_data_win_multi(
        &[CodeEntry {
            name: code_name.to_string(),
            bytecode: program.bytecode.data.clone(),
        }],
        &program
            .variables
            .iter()
            .map(|v| v.name.clone())
            .collect::<Vec<String>>(),
        &program
            .functions
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>(),
    )
}

/// Same as [`build_data_win`], but for more than one compiled script. The
/// first entry (`code[0]`) is the one wired up as the room's creation
/// code; the rest just ride along in the CODE chunk for a runner that
/// looks scripts up by name/index itself.
pub fn build_data_win_multi(
    code: &[CodeEntry],
    variables: &[String],
    functions: &[String],
) -> Vec<u8> {
    let mut pool = StringPool::new();

    let creation_code_id: i32 = if code.is_empty() { -1 } else { 0 };

    // Chunk order matches the conventional GameMaker data.win layout.
    let mut chunks: Vec<ChunkBuilder> = vec![
        build_gen8(&mut pool),
        build_optn(),
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
        build_room(&mut pool, creation_code_id),
        ChunkBuilder::empty_list("TPAG"),
        build_code(&mut pool, code),
        build_vari(&mut pool, variables),
        build_func(&mut pool, functions),
        ChunkBuilder::new("STRG"), // placeholder; rebuilt below once base offset is known
        ChunkBuilder::empty_list("TXTR"),
        ChunkBuilder::empty_list("AUDO"),
    ];
    let strg_idx = chunks.iter().position(|c| &c.name == b"STRG").unwrap();

    // --- Stage 1: lay out every chunk before STRG (their sizes don't
    // depend on where strings end up, only on the fixed 4-byte
    // placeholders already written). ---
    let mut base_offsets = vec![0usize; chunks.len()];
    let mut cursor = 8usize; // past "FORM" + u32 length
    for i in 0..strg_idx {
        base_offsets[i] = cursor + 8; // skip this chunk's own 8-byte header
        cursor += 8 + chunks[i].data.len();
    }
    base_offsets[strg_idx] = cursor + 8;

    // --- Stage 2: build the real STRG chunk now that we know its base
    // file offset, computing each string's length-prefix offset (for
    // STRG's own pointer table) and character offset (for every other
    // chunk's string references). ---
    let strg_base = base_offsets[strg_idx];
    let mut strg = ChunkBuilder::new("STRG");
    strg.u32(pool.strings.len() as u32);
    let ptr_table_pos = strg.pos();
    for _ in &pool.strings {
        strg.u32(0); // placeholder pointer table entries, filled below
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

    // --- Stage 3: lay out everything after STRG using its real size. ---
    cursor = strg_base + chunks[strg_idx].data.len();
    for i in (strg_idx + 1)..chunks.len() {
        base_offsets[i] = cursor + 8;
        cursor += 8 + chunks[i].data.len();
    }
    let total_file_size = cursor;

    // --- Stage 4: back-fill every reserved placeholder across all
    // chunks now that both string offsets and chunk base offsets are
    // known. ---
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

    // --- Stage 5: concatenate everything into the final file. ---
    let mut out = Vec::with_capacity(total_file_size);
    out.extend_from_slice(b"FORM");
    out.extend_from_slice(&((total_file_size - 8) as u32).to_le_bytes());
    for c in &chunks {
        out.extend_from_slice(&c.name);
        out.extend_from_slice(&(c.data.len() as u32).to_le_bytes());
        out.extend_from_slice(&c.data);
    }
    debug_assert_eq!(out.len(), total_file_size);
    out
}

/// Convenience wrapper: build and write straight to disk.
pub fn write_data_win(path: impl AsRef<Path>, code_name: &str, program: Program) -> io::Result<()> {
    let bytes = build_data_win(code_name, program);
    std::fs::write(path, bytes)
}

// ---------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------

/// One patch that must be back-filled once final file layout is known.
#[derive(Clone, Copy)]
enum Patch {
    /// (byte offset within this chunk's data, string id) -> absolute
    /// file offset of that string's *characters*.
    Str(usize, usize),
    /// (byte offset within this chunk's data, target offset relative to
    /// the start of this SAME chunk's data) -> absolute file offset
    /// (chunk_base + target).
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

    /// A chunk consisting of nothing but a single zero "count" field -
    /// the shape every list-bearing-but-empty chunk in this file takes.
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

    /// Reserve 4 bytes now; patched later to the absolute file offset of
    /// `s`'s characters once the STRG chunk has been laid out.
    fn str_ref(&mut self, pool: &mut StringPool, s: &str) {
        let id = pool.intern(s);
        let pos = self.pos();
        self.u32(0);
        self.patches.push(Patch::Str(pos, id));
    }

    /// Reserve 4 bytes now; returns the reserved position so the caller
    /// can push a `Patch::Local` once the target offset is known (which
    /// may be immediately, or after writing more of the chunk).
    fn local_ref_placeholder(&mut self) -> usize {
        let pos = self.pos();
        self.u32(0);
        pos
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

// ---------------------------------------------------------------------
// Chunk builders
// ---------------------------------------------------------------------

fn build_gen8(pool: &mut StringPool) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("GEN8");

    c.u8(0); // isDebuggerDisabled
    c.u8(WAD_VERSION);
    c.zero_bytes(2); // padding

    c.str_ref(pool, "mygame"); // fileName
    c.str_ref(pool, "Configs\\Default"); // config
    c.u32(0); // lastObj (no objects defined)
    c.u32(0); // lastTile (no tiles defined)
    c.u32(0x1234_5678); // gameID (arbitrary)
    c.zero_bytes(16); // directPlayGuid

    c.str_ref(pool, "MyGame"); // name
    c.u32(1); // major
    c.u32(4); // minor
    c.u32(9999); // release
    c.u32(0); // build
    c.u32(640); // defaultWindowWidth
    c.u32(480); // defaultWindowHeight
    c.u32(0); // info (bitflags)
    c.u32(0); // licenseCRC32
    c.zero_bytes(16); // licenseMD5

    // wadVersion (13) is > 12, so we take the "full" tail below rather
    // than the compact 12-and-under branch.
    c.u64(0); // timestamp
    c.str_ref(pool, "My Game"); // displayName
    c.u64(0); // activeTargets
    c.u64(0); // functionClassifications
    c.i32(0); // steamAppID
    // wadVersion < 14, so no debuggerPort field here.

    c.u32(1); // roomOrderCount
    c.i32(0); // roomOrder[0] -> our single room, index 0
    // major == 1, so no GMS2 random-seed / FPS / GUID tail.

    c
}

fn build_optn() -> ChunkBuilder {
    let mut c = ChunkBuilder::new("OPTN");

    // shaderExtensionFlag == 0x80000000 selects the "new" bitflag layout,
    // which is far less fiddly to emit than the legacy bool-per-field one.
    c.u32(0x8000_0000);
    c.i32(1); // shaderExtVersion

    c.u64(0x10); // info bitflags (0x10 = ShowCursor)
    c.i32(0); // scale
    c.u32(0); // windowColor
    c.u32(32); // colorDepth
    c.u32(0); // resolution
    c.u32(60); // frequency
    c.u32(1); // vertexSync
    c.u32(0); // priority
    c.u32(0); // backImage
    c.u32(0); // frontImage
    c.u32(0); // loadImage
    c.u32(255); // loadAlpha

    // wadVersion(13) > 8, so the Constants SimpleList is present; empty.
    c.u32(0); // constantCount

    c
}

fn build_room(pool: &mut StringPool, creation_code_id: i32) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("ROOM");

    c.u32(1); // one room
    let ptr0_pos = c.local_ref_placeholder();
    let room_start = c.pos();
    c.patches.push(Patch::Local(ptr0_pos, room_start));

    c.str_ref(pool, "room0"); // name
    c.str_ref(pool, ""); // caption
    c.u32(640); // width
    c.u32(480); // height
    c.u32(30); // speed
    c.bool32(false); // persistent
    c.u32(0x00FF_FFFF); // backgroundColor
    c.bool32(true); // drawBackgroundColor
    c.i32(creation_code_id); // creationCodeId - runs your bytecode on room load
    c.u32(0); // flags

    let bg_off_pos = c.local_ref_placeholder();
    let view_off_pos = c.local_ref_placeholder();
    let obj_off_pos = c.local_ref_placeholder();
    let tile_off_pos = c.local_ref_placeholder();

    c.bool32(false); // world
    c.u32(0); // top
    c.u32(0); // left
    c.u32(640); // right
    c.u32(480); // bottom
    c.f32(0.0); // gravityX
    c.f32(10.0); // gravityY
    c.f32(0.1); // metersPerPixel
    // major == 1, so no layersFileOffset / sequencesPtr tail.

    let bg_list_off = c.pos();
    c.u32(0); // backgrounds: 0 entries (reader zero-fills all 8 slots)
    let view_list_off = c.pos();
    c.u32(0); // views: 0 entries
    let obj_list_off = c.pos();
    c.u32(0); // game objects: 0 entries
    let tile_list_off = c.pos();
    c.u32(0); // tiles: 0 entries

    c.patches.push(Patch::Local(bg_off_pos, bg_list_off));
    c.patches.push(Patch::Local(view_off_pos, view_list_off));
    c.patches.push(Patch::Local(obj_off_pos, obj_list_off));
    c.patches.push(Patch::Local(tile_off_pos, tile_list_off));

    c
}

/// CODE chunk: a PointerList of entries. Uses the wadVersion<=14 ("old
/// format") per-entry layout, which is just `name, length, <raw bytes>`
/// with no locals/arguments/relative-address header.
fn build_code(pool: &mut StringPool, code: &[CodeEntry]) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("CODE");

    c.u32(code.len() as u32);
    let ptr_positions: Vec<usize> = (0..code.len()).map(|_| c.local_ref_placeholder()).collect();

    for (entry, ptr_pos) in code.iter().zip(ptr_positions) {
        let entry_start = c.pos();
        c.patches.push(Patch::Local(ptr_pos, entry_start));

        c.str_ref(pool, &entry.name);
        c.u32(entry.bytecode.len() as u32); // length
        c.bytes(&entry.bytecode); // instructions, inline (oldFormat)
    }

    c
}

/// VARI chunk, wadVersion<=14 ("old format"): no header, and each entry is
/// exactly 12 bytes (name, occurrences, firstAddress - no instanceType or
/// varID, those only exist in the newer per-entry layout).
///
/// `occurrences`/`firstAddress` drive the runtime's bytecode-patching pass
/// that resolves variable-name references it finds while scanning code.
/// If gmlc already resolves variable IDs at compile time (baking them
/// directly into the emitted bytecode) rather than relying on that patch
/// chain, `occurrences = 0` / `firstAddress = -1` (the "nothing to patch"
/// sentinel) is the right, inert choice used here. If your runner instead
/// expects to walk this chain, thread the real occurrence/address data
/// through from gmlc's compiler internals instead.
fn build_vari(pool: &mut StringPool, variables: &[String]) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("VARI");
    for name in variables {
        c.str_ref(pool, name);
        c.u32(0); // occurrences
        c.i32(-1); // firstAddress sentinel (no patch chain)
    }
    c
}

fn build_func(pool: &mut StringPool, functions: &[String]) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("FUNC");
    for name in functions {
        c.str_ref(pool, name);
        c.u32(0); // occurrences
        c.i32(-1); // firstAddress sentinel (no patch chain)
    }
    c
}

// ---------------------------------------------------------------------
// Extending this further
// ---------------------------------------------------------------------
// - Multiple scripts: already supported via `build_data_win_multi`/
//   `CodeEntry`; only `code[0]` is wired to the room's creation code.
// - Function references (built-in/script calls your bytecode makes): add
//   a `build_func` mirroring `build_vari` and give `build_data_win_multi`
//   a `functions: &[String]` parameter - currently FUNC is left empty
//   (0 bytes), which the wadVersion<=14 branch reads as "zero functions".
// - Real occurrence-patching data for VARI/FUNC: if your runner resolves
//   variables lazily via this chain rather than at compile time, extend
//   `CodeEntry`/a new `VariableEntry` type to carry the actual
//   (occurrences, firstAddress) your compiler already tracks.

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

        // Room's creationCodeId (5th field after name/caption strptrs,
        // width, height, speed, persistent, bgcolor, drawbg -> offset
        // computed below) points at our one CODE entry (index 0).
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
