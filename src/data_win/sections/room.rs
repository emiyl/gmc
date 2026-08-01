use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::layout::WadLayout;
use crate::data_win::model::CompiledRoom;
use crate::data_win::string_pool::StringPool;

pub fn build(rooms: &[CompiledRoom], pool: &mut StringPool, layout: &WadLayout) -> ChunkBuilder {
    let mut chunk = ChunkBuilder::new("ROOM");

    chunk.u32(rooms.len() as u32);
    let room_ptrs = (0..rooms.len())
        .map(|_| chunk.local_ref_placeholder())
        .collect::<Vec<_>>();

    for (room, room_ptr) in rooms.iter().zip(room_ptrs.into_iter()) {
        let room_start = chunk.pos();
        chunk.local_ref_set(room_ptr, room_start);

        chunk.str_ref(pool, &room.name);
        chunk.str_ref(pool, &room.caption);
        chunk.u32(room.width);
        chunk.u32(room.height);
        chunk.u32(room.speed);
        chunk.bool32(room.persistent);
        chunk.u32(room.background_color);
        chunk.bool32(room.draw_background_color);
        chunk.i32(room.creation_code_id);
        chunk.u32(room.flags);

        let backgrounds_pos = chunk.local_ref_placeholder();
        let views_pos = chunk.local_ref_placeholder();
        let game_objects_pos = chunk.local_ref_placeholder();
        let tiles_pos = chunk.local_ref_placeholder();

        chunk.bool32(room.world);
        chunk.u32(room.top);
        chunk.u32(room.left);
        chunk.u32(room.right);
        chunk.u32(room.bottom);
        chunk.f32(room.gravity_x);
        chunk.f32(room.gravity_y);
        chunk.f32(room.meters_per_pixel);

        let layers_pos = if layout.gms2_room_tail {
            Some(chunk.local_ref_placeholder())
        } else {
            None
        };

        if layout.gms2_room_tail && layout.gms2_3_sequences {
            chunk.local_ref_placeholder();
        }

        let backgrounds_start = chunk.pos();
        chunk.u32(room.background_entry_count);

        let views_start = chunk.pos();
        chunk.u32(room.view_entry_count);

        let game_objects_start = chunk.pos();
        chunk.u32(room.instances.len() as u32);
        let object_ptrs = (0..room.instances.len())
            .map(|_| chunk.local_ref_placeholder())
            .collect::<Vec<_>>();

        for (instance, instance_ptr) in room.instances.iter().zip(object_ptrs.into_iter()) {
            let instance_start = chunk.pos();
            chunk.local_ref_set(instance_ptr, instance_start);

            chunk.i32(instance.x as i32);
            chunk.i32(instance.y as i32);
            chunk.i32(instance.object_id as i32);
            chunk.u32(instance.id);
            chunk.i32(instance.creation_code_id);
            chunk.f32(instance.scale_x);
            chunk.f32(instance.scale_y);
            chunk.f32(instance.image_speed);
            chunk.i32(instance.image_index);
            chunk.u32(instance.color);
            chunk.f32(instance.rotation);

            // WAD16+ stores preCreateCode after rotation.
            if layout.wad_version >= 16 {
                chunk.i32(instance.pre_create_code);
            }
        }

        let tiles_start = chunk.pos();
        chunk.u32(room.tile_entry_count);

        chunk.local_ref_set(backgrounds_pos, backgrounds_start);
        chunk.local_ref_set(views_pos, views_start);
        chunk.local_ref_set(game_objects_pos, game_objects_start);
        chunk.local_ref_set(tiles_pos, tiles_start);

        if let Some(layers_pos) = layers_pos {
            let layers_start = chunk.pos();
            chunk.u32(room.layer_entry_count);
            chunk.local_ref_set(layers_pos, layers_start);
        }
    }

    chunk
}
