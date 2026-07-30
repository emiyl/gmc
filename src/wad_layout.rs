#[derive(Clone, Copy, Debug)]
pub struct WadLayout {
    pub wad_version: u8,

    // GameMaker product version stamped into GEN8. This is a separate
    // concept from `wad_version` in the real format (the parser derives
    // it from a mix of GEN8 and later chunk contents), but since we fully
    // control the output we just pick values consistent with the
    // wad_version bracket we're emitting.
    pub major: u32,
    pub minor: u32,
    pub release: u32,
    pub build: u32,

    // --- GEN8 shape ---
    pub compact_gen8: bool,       // wad_version <= 8: tiny WAD8 layout
    pub has_display_name: bool,   // wad_version >= 10 (absent for <=9)
    pub has_active_targets: bool, // wad_version >= 11
    pub has_function_classifications: bool, // wad_version >= 12
    pub has_debugger_port: bool,  // wad_version >= 14
    pub timestamp_is_64bit: bool, // wad_version >= 13 (else 32-bit + padding)

    // --- OPTN shape ---
    pub has_constants: bool, // wad_version > 8

    // --- CODE / VARI / FUNC shape ---
    pub old_code_format: bool, // wad_version <= 14: flat, header-free entries

    // --- ROOM shape ---
    pub gms2_room_tail: bool, // major >= 2: room gets a layersFileOffset field
    pub gms2_3_sequences: bool, // major.minor >= 2.3: room also gets a sequencesPtr field
}

impl WadLayout {
    pub fn for_version(wad_version: u8) -> WadLayout {
        // This match is the "switch case" that picks the on-disk shape
        // for each historical wadVersion bracket, mirroring the branches
        // in the C reference parser's parseGEN8/parseOPTN/parseCODE/
        // parseVARI/parseFUNC/parseROOM.
        match wad_version {
            0..=8 => WadLayout {
                wad_version,
                major: 1,
                minor: 0,
                release: 0,
                build: 198,
                compact_gen8: true,
                has_display_name: false,
                has_active_targets: false,
                has_function_classifications: false,
                has_debugger_port: false,
                timestamp_is_64bit: false,
                has_constants: false,
                old_code_format: true,
                gms2_room_tail: false,
                gms2_3_sequences: false,
            },
            9 => WadLayout {
                wad_version,
                major: 1,
                minor: 0,
                release: 0,
                build: 300,
                compact_gen8: false,
                has_display_name: false,
                has_active_targets: false,
                has_function_classifications: false,
                has_debugger_port: false,
                timestamp_is_64bit: false,
                has_constants: true,
                old_code_format: true,
                gms2_room_tail: false,
                gms2_3_sequences: false,
            },
            10..=12 => WadLayout {
                wad_version,
                major: 1,
                minor: 0,
                release: 0,
                build: 400,
                compact_gen8: false,
                has_display_name: true,
                has_active_targets: wad_version >= 11,
                has_function_classifications: wad_version >= 12,
                has_debugger_port: false,
                timestamp_is_64bit: false,
                has_constants: true,
                old_code_format: true,
                gms2_room_tail: false,
                gms2_3_sequences: false,
            },
            13 => WadLayout {
                wad_version,
                major: 1,
                minor: 4,
                release: 0,
                build: 0,
                compact_gen8: false,
                has_display_name: true,
                has_active_targets: true,
                has_function_classifications: true,
                has_debugger_port: false,
                timestamp_is_64bit: true,
                has_constants: true,
                old_code_format: true,
                gms2_room_tail: false,
                gms2_3_sequences: false,
            },
            14 => WadLayout {
                wad_version,
                major: 1,
                minor: 4,
                release: 9,
                build: 9999,
                compact_gen8: false,
                has_display_name: true,
                has_active_targets: true,
                has_function_classifications: true,
                has_debugger_port: true,
                timestamp_is_64bit: true,
                has_constants: true,
                old_code_format: true,
                gms2_room_tail: false,
                gms2_3_sequences: false,
            },
            15..=16 => WadLayout {
                wad_version,
                major: 2,
                minor: 0,
                release: 0,
                build: 0,
                compact_gen8: false,
                has_display_name: true,
                has_active_targets: true,
                has_function_classifications: true,
                has_debugger_port: true,
                timestamp_is_64bit: true,
                has_constants: true,
                old_code_format: false,
                gms2_room_tail: true,
                gms2_3_sequences: false,
            },
            _ => WadLayout {
                // 17+
                wad_version,
                major: 2,
                minor: 3,
                release: 0,
                build: 0,
                compact_gen8: false,
                has_display_name: true,
                has_active_targets: true,
                has_function_classifications: true,
                has_debugger_port: true,
                timestamp_is_64bit: true,
                has_constants: true,
                old_code_format: false,
                gms2_room_tail: true,
                gms2_3_sequences: true,
            },
        }
    }
}
