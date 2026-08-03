mod instance;
mod layer;

pub use layer::LayerTrait;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use super::{ResourceId, ResourceTrait};
use crate::project::formatter::format_gamemaker_json;
use instance::Instance;
use layer::{BackgroundLayer, InstanceLayer, Layer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
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
    pub instance_creation_order: Vec<ResourceId>,

    #[serde(rename = "isDnd")]
    pub is_dnd: bool,

    pub layers: Vec<Layer>,

    pub name: String,

    pub parent: ResourceId,

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

impl Default for Room {
    fn default() -> Self {
        Room {
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
            parent: ResourceId {
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

impl ResourceTrait for Room {
    fn name(&self) -> &str {
        &self.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize Room");
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;
        Ok(())
    }

    fn default_path(&self) -> String {
        format!("rooms/{}/{}.yy", self.name, self.name)
    }
}

impl Room {
    pub fn new(name: &str, parent: ResourceId) -> Self {
        Room {
            name: name.to_string(),
            percent_name: name.to_string(),
            parent,
            ..Room::default()
        }
    }

    pub fn load(value: Value) -> std::io::Result<Self> {
        let room: Room = serde_json::from_value(value).expect("Failed to deserialize Room");
        Ok(room)
    }

    pub fn add_instance_layer(&mut self, layer_name: &str) -> &mut InstanceLayer {
        let new_layer = Layer::instance_layer(layer_name, self.layers.len() as i32);
        self.layers.push(new_layer);
        if let Layer::Instance(layer) = self.layers.last_mut().unwrap() {
            layer
        } else {
            panic!("Last layer is not an instance layer");
        }
    }

    pub fn add_background_layer(&mut self, layer_name: &str) -> &mut BackgroundLayer {
        let new_layer = Layer::background_layer(layer_name, self.layers.len() as i32);
        self.layers.push(new_layer);
        if let Layer::Background(layer) = self.layers.last_mut().unwrap() {
            layer
        } else {
            panic!("Last layer is not a background layer");
        }
    }

    pub fn add_instance(&mut self, room_id: ResourceId, object_id: ResourceId, x: f32, y: f32) {
        let instance = Instance::new(object_id, x, y);

        let resource_id = ResourceId {
            name: instance.name.clone(),
            path: room_id.path.clone(),
        };

        if let Some(Layer::Instance(layer)) = self
            .layers
            .iter_mut()
            .find(|layer| matches!(layer, Layer::Instance(_)))
        {
            layer.instances.push(instance);
        } else {
            let layer = self.add_instance_layer("Instances");
            layer.instances.push(instance);
        }

        self.instance_creation_order.push(resource_id);
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
