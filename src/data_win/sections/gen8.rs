use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::layout::WadLayout;
use crate::data_win::model::Gen8;
use crate::data_win::string_pool::StringPool;

pub fn build(gen8: &Gen8, pool: &mut StringPool, layout: &WadLayout) -> ChunkBuilder {
    let mut chunk = ChunkBuilder::new("GEN8");

    chunk.u8(gen8.is_debugger_disabled as u8);
    chunk.u8(layout.wad_version);
    chunk.zero_bytes(2);

    if layout.compact_gen8 {
        chunk.str_ref(pool, &gen8.file_name);
        chunk.u32(gen8.last_obj);
        chunk.u32(gen8.last_tile);
        chunk.u32(gen8.game_id);
        chunk.bytes(&gen8.direct_play_guid);
        chunk.u32(gen8.default_window_width);
        chunk.u32(gen8.default_window_height);
        chunk.u32(gen8.info);
        chunk.u32(gen8.license_crc32);
        chunk.bytes(&gen8.license_md5);
        chunk.u32(gen8.timestamp as u32);
        chunk.zero_bytes(4);
        chunk.u32(gen8.room_order.len() as u32);

        for room in &gen8.room_order {
            chunk.i32(*room);
        }

        return chunk;
    }

    chunk.str_ref(pool, &gen8.file_name);
    chunk.str_ref(pool, &gen8.config);
    chunk.u32(gen8.last_obj);
    chunk.u32(gen8.last_tile);
    chunk.u32(gen8.game_id);
    chunk.bytes(&gen8.direct_play_guid);
    chunk.str_ref(pool, &gen8.name);
    chunk.u32(layout.major);
    chunk.u32(layout.minor);
    chunk.u32(layout.release);
    chunk.u32(layout.build);
    chunk.u32(gen8.default_window_width);
    chunk.u32(gen8.default_window_height);
    chunk.u32(gen8.info);
    chunk.u32(gen8.license_crc32);
    chunk.bytes(&gen8.license_md5);

    if layout.timestamp_is_64bit {
        chunk.u64(gen8.timestamp);
        chunk.str_ref(pool, &gen8.display_name);
        chunk.u64(gen8.active_targets);
        chunk.u64(gen8.function_classifications);
        chunk.i32(gen8.steam_app_id);
        if layout.has_debugger_port {
            chunk.u32(gen8.debugger_port);
        }
    } else {
        chunk.i32(gen8.timestamp as i32);
        chunk.zero_bytes(4);
        if layout.has_display_name {
            chunk.str_ref(pool, &gen8.display_name);
        }
        if layout.has_active_targets {
            chunk.u32(gen8.active_targets as u32);
        }
        if layout.has_function_classifications {
            chunk.u32(gen8.function_classifications as u32);
        }
    }

    chunk.u32(gen8.room_order.len() as u32);
    for room in &gen8.room_order {
        chunk.i32(*room);
    }

    if layout.gms2_room_tail {
        chunk.u64(gen8.gms2_first_random);
        for random_uid in &gen8.gms2_random_uid {
            chunk.u64(*random_uid);
        }
        chunk.f32(gen8.gms2_fps);
        chunk.bool32(gen8.gms2_allow_statistics);
        chunk.bytes(&gen8.gms2_game_guid);
    }

    chunk
}
