use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::layout::WadLayout;
use crate::data_win::string_pool::StringPool;

pub fn build(function_names: &[String], pool: &mut StringPool, layout: &WadLayout) -> ChunkBuilder {
    let mut chunk = ChunkBuilder::new("FUNC");

    if !layout.old_code_format {
        chunk.u32(function_names.len() as u32);
    }

    for name in function_names {
        chunk.str_ref(pool, name);
        chunk.u32(0);
        chunk.i32(-1);
    }

    if !layout.old_code_format {
        chunk.u32(0);
    }

    chunk
}
