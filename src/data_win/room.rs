use crate::data_win::{ChunkBuilder, StringPool, WadLayout};
use crate::project::GmRoom;

/// A single room entry for the ROOM chunk.
pub struct CompiledRoom {
    pub name: String,
    pub caption: String,
    pub width: u32,
    pub height: u32,
    pub speed: u32,
    pub persistent: bool,
    pub background_color: u32,
    pub draw_background_color: bool,
    /// Index into the CODE chunk whose bytecode runs when this room loads,
    /// or `-1` for none.
    pub creation_code_id: i32,
    pub flags: u32,
    pub world: bool,
    pub top: u32,
    pub left: u32,
    pub right: u32,
    pub bottom: u32,
    pub gravity_x: f32,
    pub gravity_y: f32,
    pub meters_per_pixel: f32,
}

impl Default for CompiledRoom {
    fn default() -> Self {
        CompiledRoom {
            name: "room0".to_string(),
            caption: String::new(),
            width: 640,
            height: 480,
            speed: 30,
            persistent: false,
            background_color: 0x00FF_FFFF,
            draw_background_color: true,
            creation_code_id: -1,
            flags: 0,
            world: false,
            top: 0,
            left: 0,
            right: 640,
            bottom: 480,
            gravity_x: 0.0,
            gravity_y: 10.0,
            meters_per_pixel: 0.1,
        }
    }
}

impl CompiledRoom {
    pub fn from_gmroom(room: GmRoom) -> Self {
        CompiledRoom {
            name: room.name,
            width: room.room_settings.width,
            height: room.room_settings.height,
            creation_code_id: -1, // This will be set when adding code
            flags: 0,             // This can be set based on GmRoom properties if needed
            world: room.physics_settings.physics_world,
            gravity_x: room.physics_settings.physics_world_gravity_x,
            gravity_y: room.physics_settings.physics_world_gravity_y,
            meters_per_pixel: room.physics_settings.physics_world_pix_to_metres,
            ..Default::default()
        }
    }
}

pub fn serialize_rooms(
    rooms: &[CompiledRoom],
    pool: &mut StringPool,
    layout: &WadLayout,
) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("ROOM");

    c.u32(rooms.len() as u32);
    let ptr_positions: Vec<usize> = (0..rooms.len())
        .map(|_| c.local_ref_placeholder())
        .collect();

    for (room, ptr_pos) in rooms.iter().zip(ptr_positions) {
        let room_start = c.pos();
        c.local_ref_set(ptr_pos, room_start);

        c.str_ref(pool, &room.name);
        c.str_ref(pool, &room.caption);
        c.u32(room.width);
        c.u32(room.height);
        c.u32(room.speed);
        c.bool32(room.persistent);
        c.u32(room.background_color);
        c.bool32(room.draw_background_color);
        c.i32(room.creation_code_id);
        c.u32(room.flags);

        let bg_off = c.local_ref_placeholder();
        let view_off = c.local_ref_placeholder();
        let obj_off = c.local_ref_placeholder();
        let tile_off = c.local_ref_placeholder();

        c.bool32(room.world);
        c.u32(room.top);
        c.u32(room.left);
        c.u32(room.right);
        c.u32(room.bottom);
        c.f32(room.gravity_x);
        c.f32(room.gravity_y);
        c.f32(room.meters_per_pixel);

        let layers_off_pos = if layout.gms2_room_tail {
            Some(c.local_ref_placeholder())
        } else {
            None
        };
        if layout.gms2_room_tail && layout.gms2_3_sequences {
            c.local_ref_placeholder(); // sequencesPtr: left at 0, unused when layerCount == 0
        }

        let bg_list = c.pos();
        c.u32(0); // 0 backgrounds
        let view_list = c.pos();
        c.u32(0); // 0 views
        let obj_list = c.pos();
        c.u32(0); // 0 objects
        let tile_list = c.pos();
        c.u32(0); // 0 tiles

        c.local_ref_set(bg_off, bg_list);
        c.local_ref_set(view_off, view_list);
        c.local_ref_set(obj_off, obj_list);
        c.local_ref_set(tile_off, tile_list);

        if let Some(layers_off_pos) = layers_off_pos {
            let layers_list = c.pos();
            c.u32(0); // 0 layers
            c.local_ref_set(layers_off_pos, layers_list);
        }
    }

    c
}
