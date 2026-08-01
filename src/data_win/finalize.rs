use crate::data_win::chunk::{ChunkBuilder, Patch};
use crate::data_win::string_pool::StringPool;

pub fn build_form(chunks: &mut Vec<ChunkBuilder>, pool: &StringPool) -> Vec<u8> {
    let strg_index = chunks
        .iter()
        .position(|chunk| &chunk.name == b"STRG")
        .expect("STRG chunk is required");

    let mut base_offsets = vec![0usize; chunks.len()];
    let mut cursor = 8usize;

    for index in 0..strg_index {
        base_offsets[index] = cursor + 8;
        cursor += 8 + chunks[index].data.len();
    }

    base_offsets[strg_index] = cursor + 8;
    let strg_base = base_offsets[strg_index];

    let mut strg_chunk = ChunkBuilder::new("STRG");
    strg_chunk.u32(pool.values().len() as u32);

    let pointer_table_pos = strg_chunk.pos();
    for _ in pool.values() {
        strg_chunk.u32(0);
    }

    let mut char_offsets = Vec::with_capacity(pool.values().len());
    for (index, value) in pool.values().iter().enumerate() {
        let len_prefix_rel = strg_chunk.pos();
        strg_chunk.u32(value.len() as u32);
        strg_chunk.bytes(value.as_bytes());
        strg_chunk.u8(0);

        let abs_len_prefix = strg_base + len_prefix_rel;
        strg_chunk.set_u32(pointer_table_pos + index * 4, abs_len_prefix as u32);
        char_offsets.push((abs_len_prefix + 4) as u32);
    }

    chunks[strg_index] = strg_chunk;

    cursor = strg_base + chunks[strg_index].data.len();
    for index in (strg_index + 1)..chunks.len() {
        base_offsets[index] = cursor + 8;
        cursor += 8 + chunks[index].data.len();
    }

    let total_file_size = cursor;

    for (chunk_index, chunk) in chunks.iter_mut().enumerate() {
        let patches = chunk.patches.clone();
        for patch in patches {
            match patch {
                Patch::Str(pos, string_id) => {
                    chunk.set_u32(pos, char_offsets[string_id]);
                }
                Patch::StrIndex(pos, string_id) => {
                    chunk.set_u32(pos, string_id as u32);
                }
                Patch::Local(pos, rel_target) => {
                    chunk.set_u32(pos, (base_offsets[chunk_index] + rel_target) as u32);
                }
            }
        }
    }

    let mut output = Vec::with_capacity(total_file_size);
    output.extend_from_slice(b"FORM");
    output.extend_from_slice(&((total_file_size - 8) as u32).to_le_bytes());

    for chunk in chunks.iter() {
        output.extend_from_slice(&chunk.name);
        output.extend_from_slice(&(chunk.data.len() as u32).to_le_bytes());
        output.extend_from_slice(&chunk.data);
    }

    debug_assert_eq!(output.len(), total_file_size);
    output
}
