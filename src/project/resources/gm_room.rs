//! Parser and data model for GameMaker `.yy` room files (e.g. `Room1.yy`).
//!
//! `.yy` files are JSON, but GameMaker's own writer leaves trailing commas
//! before closing `}` / `]`, which is not valid strict JSON. [`parse_str`]
//! and [`parse_file`] strip those before handing the text to `serde_json`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::Read;
use std::path::Path;

use crate::project::{
    formatter::format_gamemaker_json,
    resources::gm_room_layer::{Instance, Layer},
};

use super::{Resource, ResourceRef};

// ---------------------------------------------------------------------
// Top-level room
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GmRoom {
    #[serde(rename = "$GMRoom")]
    pub gm_room: String,

    #[serde(rename = "%Name")]
    pub percent_name: String,

    #[serde(rename = "creationCodeFile")]
    pub creation_code_file: String,

    #[serde(rename = "inheritCode")]
    pub inherit_code: bool,

    #[serde(rename = "inheritCreationOrder")]
    pub inherit_creation_order: bool,

    #[serde(rename = "inheritLayers")]
    pub inherit_layers: bool,

    #[serde(rename = "instanceCreationOrder")]
    pub instance_creation_order: Vec<ResourceRef>,

    #[serde(rename = "isDnd")]
    pub is_dnd: bool,

    pub layers: Vec<Layer>,

    pub name: String,

    pub parent: ResourceRef,

    #[serde(rename = "parentRoom")]
    pub parent_room: Option<Value>,

    #[serde(rename = "physicsSettings")]
    pub physics_settings: PhysicsSettings,

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    #[serde(rename = "roomSettings")]
    pub room_settings: RoomSettings,

    #[serde(rename = "sequenceId")]
    pub sequence_id: Option<Value>,

    pub views: Vec<View>,

    #[serde(rename = "viewSettings")]
    pub view_settings: ViewSettings,

    pub volume: f64,
}

impl Default for GmRoom {
    fn default() -> Self {
        GmRoom {
            gm_room: "v1".to_string(),
            percent_name: "Room1".to_string(),
            creation_code_file: String::new(),
            inherit_code: false,
            inherit_creation_order: false,
            inherit_layers: false,
            instance_creation_order: Vec::new(),
            is_dnd: false,
            layers: vec![
                Layer::instance_layer("Instances", 0),
                Layer::background_layer("Background", 100),
            ],
            name: "Room1".to_string(),
            parent: ResourceRef {
                name: "BLANK GAME".to_string(),
                path: "BLANK GAME.yyp".to_string(),
            },
            parent_room: None,
            physics_settings: PhysicsSettings::default(),
            resource_type: "GMRoom".to_string(),
            resource_version: "2.0".to_string(),
            room_settings: RoomSettings::default(),
            sequence_id: None,
            views: (0..8).map(|_| View::default()).collect(),
            view_settings: ViewSettings::default(),
            volume: 1.0,
        }
    }
}

impl Resource for GmRoom {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_path(&self) -> &str {
        let path = &self.parent.path;
        println!("Parent path: {}", path);
        // &format!("rooms/{}/{}.yy", self.name, self.name)
        path
    }
}

impl GmRoom {
    pub fn new(name: &str, parent: ResourceRef) -> Self {
        GmRoom {
            name: name.to_string(),
            percent_name: name.to_string(),
            parent,
            ..GmRoom::default()
        }
    }

    pub fn add_instance(&mut self, object_ref: ResourceRef, x: f32, y: f32) {
        let instance = Instance::new(object_ref, x, y);
        if let Some(Layer::Instance(instance_layer)) = self.layers.iter_mut().find(|layer| {
            if let Layer::Instance(instance_layer) = layer {
                instance_layer.display_name == "Instances"
            } else {
                false
            }
        }) {
            let instance_ref = ResourceRef {
                name: instance.name.clone(),
                path: format!("rooms/{}/{}.yy", self.name, self.name),
            };
            instance_layer.add_instance(instance);
            self.instance_creation_order.push(instance_ref);
        } else {
            eprintln!("No instance layer found to add the instance.");
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, YyError> {
        // read as json5
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let value: Value = json5::from_str(&contents).expect("Failed to parse JSON5");
        let room: GmRoom = serde_json::from_value(value).expect("Failed to deserialize GmRoom");
        Ok(room)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), YyError> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------
// Physics / room / view settings
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicsSettings {
    #[serde(rename = "inheritPhysicsSettings")]
    pub inherit_physics_settings: bool,
    #[serde(rename = "PhysicsWorld")]
    pub physics_world: bool,
    #[serde(rename = "PhysicsWorldGravityX")]
    pub physics_world_gravity_x: f32,
    #[serde(rename = "PhysicsWorldGravityY")]
    pub physics_world_gravity_y: f32,
    #[serde(rename = "PhysicsWorldPixToMetres")]
    pub physics_world_pix_to_metres: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        PhysicsSettings {
            inherit_physics_settings: false,
            physics_world: false,
            physics_world_gravity_x: 0.0,
            physics_world_gravity_y: 10.0,
            physics_world_pix_to_metres: 0.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomSettings {
    #[serde(rename = "Height")]
    pub height: u32,
    #[serde(rename = "inheritRoomSettings")]
    pub inherit_room_settings: bool,
    pub persistent: bool,
    #[serde(rename = "Width")]
    pub width: u32,
}

impl Default for RoomSettings {
    fn default() -> Self {
        RoomSettings {
            height: 768,
            width: 1366,
            inherit_room_settings: false,
            persistent: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewSettings {
    #[serde(rename = "clearDisplayBuffer")]
    pub clear_display_buffer: bool,
    #[serde(rename = "clearViewBackground")]
    pub clear_view_background: bool,
    #[serde(rename = "enableViews")]
    pub enable_views: bool,
    #[serde(rename = "inheritViewSettings")]
    pub inherit_view_settings: bool,
}

impl Default for ViewSettings {
    fn default() -> Self {
        ViewSettings {
            clear_display_buffer: true,
            clear_view_background: false,
            enable_views: false,
            inherit_view_settings: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    pub hborder: i64,
    pub hport: i64,
    pub hspeed: i64,
    pub hview: i64,
    pub inherit: bool,
    #[serde(rename = "objectId")]
    pub object_id: Option<Value>,
    pub vborder: i64,
    pub visible: bool,
    pub vspeed: i64,
    pub wport: i64,
    pub wview: i64,
    pub xport: i64,
    pub xview: i64,
    pub yport: i64,
    pub yview: i64,
}

impl Default for View {
    fn default() -> Self {
        View {
            hborder: 32,
            hport: 768,
            hspeed: -1,
            hview: 768,
            inherit: false,
            object_id: None,
            vborder: 32,
            visible: false,
            vspeed: -1,
            wport: 1366,
            wview: 1366,
            xport: 0,
            xview: 0,
            yport: 0,
            yview: 0,
        }
    }
}

// ---------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------

#[derive(Debug)]
pub enum YyError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for YyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YyError::Io(e) => write!(f, "I/O error reading .yy file: {e}"),
            YyError::Json(e) => write!(f, "JSON error parsing .yy file: {e}"),
        }
    }
}

impl std::error::Error for YyError {}

impl From<std::io::Error> for YyError {
    fn from(e: std::io::Error) -> Self {
        YyError::Io(e)
    }
}

impl From<serde_json::Error> for YyError {
    fn from(e: serde_json::Error) -> Self {
        YyError::Json(e)
    }
}
