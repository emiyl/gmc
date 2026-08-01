use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::layout::WadLayout;
use crate::data_win::model::Optn;

pub fn build(optn: &Optn, layout: &WadLayout) -> ChunkBuilder {
    let mut chunk = ChunkBuilder::new("OPTN");

    chunk.u32(optn.shader_extension_flag);
    chunk.i32(optn.shader_ext_version);

    chunk.u64(optn.info);
    chunk.i32(optn.scale);
    chunk.u32(optn.window_color);
    chunk.u32(optn.color_depth);
    chunk.u32(optn.resolution);
    chunk.u32(optn.frequency);
    chunk.u32(optn.vertex_sync);
    chunk.u32(optn.priority);
    chunk.u32(optn.back_image);
    chunk.u32(optn.front_image);
    chunk.u32(optn.load_image);
    chunk.u32(optn.load_alpha);

    if layout.has_constants {
        chunk.u32(optn.constant_count);
    }

    chunk
}
