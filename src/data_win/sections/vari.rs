use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::layout::WadLayout;
use crate::data_win::model::CompiledVariable;
use crate::data_win::string_pool::StringPool;

pub fn build(
    variables: &[CompiledVariable],
    pool: &mut StringPool,
    layout: &WadLayout,
    max_local_var_count: u32,
) -> ChunkBuilder {
    let mut chunk = ChunkBuilder::new("VARI");

    if layout.old_code_format {
        for variable in variables {
            chunk.str_ref(pool, &variable.name);
            chunk.u32(variable.occurrences);
            chunk.i32(variable.first_address);
        }

        return chunk;
    }

    chunk.u32(variables.len() as u32);
    chunk.u32(variables.len() as u32);
    chunk.u32(max_local_var_count);

    for variable in variables {
        chunk.str_ref(pool, &variable.name);
        chunk.i32(variable.instance_type);
        chunk.i32(variable.var_id);
        chunk.u32(variable.occurrences);
        chunk.i32(variable.first_address);
    }

    chunk
}
