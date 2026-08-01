use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::layout::WadLayout;
use crate::data_win::string_pool::StringPool;

pub fn build(variable_names: &[String], pool: &mut StringPool, layout: &WadLayout) -> ChunkBuilder {
    let mut chunk = ChunkBuilder::new("VARI");

    if layout.old_code_format {
        for name in variable_names {
            chunk.str_ref(pool, name);
            chunk.u32(0);
            chunk.i32(-1);
        }

        return chunk;
    }

    chunk.u32(variable_names.len() as u32);
    chunk.u32(variable_names.len() as u32);
    chunk.u32(0);

    for (index, name) in variable_names.iter().enumerate() {
        chunk.str_ref(pool, name);
        chunk.i32(0);
        chunk.i32(index as i32);
        chunk.u32(0);
        chunk.i32(-1);
    }

    chunk
}
