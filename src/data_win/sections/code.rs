use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::layout::WadLayout;
use crate::data_win::model::CompiledCodeEntry;
use crate::data_win::string_pool::StringPool;

pub fn build(
    code_entries: &[CompiledCodeEntry],
    pool: &mut StringPool,
    layout: &WadLayout,
) -> ChunkBuilder {
    let mut chunk = ChunkBuilder::new("CODE");

    chunk.u32(code_entries.len() as u32);
    let ptr_positions = (0..code_entries.len())
        .map(|_| chunk.local_ref_placeholder())
        .collect::<Vec<_>>();

    if layout.old_code_format {
        for (entry, ptr_pos) in code_entries.iter().zip(ptr_positions.into_iter()) {
            let entry_start = chunk.pos();
            chunk.local_ref_set(ptr_pos, entry_start);

            chunk.str_ref(pool, &entry.name);
            chunk.u32(entry.bytecode.len() as u32);
            chunk.bytes(&entry.bytecode);
        }

        return chunk;
    }

    let mut rel_addr_positions = Vec::with_capacity(code_entries.len());
    for (entry, ptr_pos) in code_entries.iter().zip(ptr_positions.iter()) {
        let entry_start = chunk.pos();
        chunk.local_ref_set(*ptr_pos, entry_start);

        chunk.str_ref(pool, &entry.name);
        chunk.u32(entry.bytecode.len() as u32);
        chunk.u16(0);
        chunk.u16(0);
        let rel_addr_field = chunk.pos();
        chunk.i32(0);
        chunk.u32(0);
        rel_addr_positions.push(rel_addr_field);
    }

    for (entry, rel_pos) in code_entries.iter().zip(rel_addr_positions.into_iter()) {
        let bytecode_start = chunk.pos();
        chunk.bytes(&entry.bytecode);

        let rel = bytecode_start as i64 - rel_pos as i64;
        chunk.set_u32(rel_pos, rel as i32 as u32);
    }

    chunk
}
