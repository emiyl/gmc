#[derive(Debug, Clone)]
pub struct Gen8 {
    pub is_debugger_disabled: bool,
    pub file_name: String,
    pub config: String,
    pub last_obj: u32,
    pub last_tile: u32,
    pub game_id: u32,
    pub direct_play_guid: [u8; 16],
    pub name: String,
    pub default_window_width: u32,
    pub default_window_height: u32,
    pub info: u32,
    pub license_crc32: u32,
    pub license_md5: [u8; 16],
    pub timestamp: u64,
    pub display_name: String,
    pub active_targets: u64,
    pub function_classifications: u64,
    pub steam_app_id: i32,
    pub debugger_port: u32,
    pub room_order: Vec<i32>,
}

impl Default for Gen8 {
    fn default() -> Self {
        Self {
            is_debugger_disabled: false,
            file_name: "mygame".to_string(),
            config: "Configs\\Default".to_string(),
            last_obj: 0,
            last_tile: 0,
            game_id: 0x1234_5678,
            direct_play_guid: [0; 16],
            name: "MyGame".to_string(),
            default_window_width: 1366,
            default_window_height: 768,
            info: 0,
            license_crc32: 0,
            license_md5: [0; 16],
            timestamp: 0,
            display_name: "My Game".to_string(),
            active_targets: 0,
            function_classifications: 0,
            steam_app_id: 0,
            debugger_port: 0,
            room_order: vec![0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Optn {
    pub info: u64,
    pub scale: i32,
    pub window_color: u32,
    pub color_depth: u32,
    pub resolution: u32,
    pub frequency: u32,
    pub vertex_sync: u32,
    pub priority: u32,
    pub back_image: u32,
    pub front_image: u32,
    pub load_image: u32,
    pub load_alpha: u32,
    pub constant_count: u32,
}

impl Default for Optn {
    fn default() -> Self {
        Self {
            info: 0x10,
            scale: 0,
            window_color: 0,
            color_depth: 32,
            resolution: 0,
            frequency: 60,
            vertex_sync: 1,
            priority: 0,
            back_image: 0,
            front_image: 0,
            load_image: 0,
            load_alpha: 255,
            constant_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompiledCodeOwner {
    Script {
        name: String,
    },
    ObjectEvent {
        object_id: u32,
        event_type: i32,
        event_num: i32,
    },
}

#[derive(Debug, Clone)]
pub struct CompiledCodeEntry {
    pub owner: CompiledCodeOwner,
    pub name: String,
    pub bytecode: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CompiledInstance {
    pub id: u32,
    pub object_id: u32,
    pub x: f32,
    pub y: f32,
    pub creation_code_id: i32,
}

#[derive(Debug, Clone)]
pub struct CompiledRoom {
    pub name: String,
    pub caption: String,
    pub width: u32,
    pub height: u32,
    pub speed: u32,
    pub persistent: bool,
    pub background_color: u32,
    pub draw_background_color: bool,
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
    pub instances: Vec<CompiledInstance>,
}

impl Default for CompiledRoom {
    fn default() -> Self {
        Self {
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
            instances: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhysicsVertex {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct CompiledEventAction {
    pub lib_id: u32,
    pub id: u32,
    pub kind: u32,
    pub use_relative: bool,
    pub is_question: bool,
    pub use_apply_to: bool,
    pub exe_type: u32,
    pub action_name: String,
    pub code_id: i32,
    pub argument_count: u32,
    pub who: i32,
    pub relative: bool,
    pub is_not: bool,
    pub unknown_always_zero: u32,
}

#[derive(Debug, Clone)]
pub struct CompiledEvent {
    pub event_num: i32,
    pub actions: Vec<CompiledEventAction>,
}

#[derive(Debug, Clone)]
pub struct CompiledObject {
    pub name: String,
    pub sprite: i32,
    pub parent: i32,
    pub mask: i32,
    pub visible: bool,
    pub managed: bool,
    pub solid: bool,
    pub depth: i32,
    pub persistent: bool,
    pub physics_angular_damping: f32,
    pub physics_density: f32,
    pub physics_friction: f32,
    pub physics_group: i32,
    pub physics_kinematic: bool,
    pub physics_linear_damping: f32,
    pub physics_object: bool,
    pub physics_restitution: f32,
    pub physics_sensor: bool,
    pub physics_shape: i32,
    pub physics_start_awake: bool,
    pub physics_vertices: Vec<PhysicsVertex>,
    pub event_lists: [Vec<CompiledEvent>; 15],
}
