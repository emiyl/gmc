use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::layout::WadLayout;
use crate::data_win::model::CompiledFunction;
use crate::data_win::string_pool::StringPool;

pub fn build(
    functions: &[CompiledFunction],
    pool: &mut StringPool,
    layout: &WadLayout,
    code_locals_count: u32,
) -> ChunkBuilder {
    let mut chunk = ChunkBuilder::new("FUNC");

    if !layout.old_code_format {
        chunk.u32(functions.len() as u32);
    }

    for function in functions {
        chunk.str_ref(pool, &function.name);
        chunk.u32(function.occurrences);
        chunk.i32(function.first_address);
    }

    if !layout.old_code_format {
        chunk.u32(code_locals_count);
    }

    chunk
}
