use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::Path;

use crate::project::formatter::format_gamemaker_json;

use super::{Resource, ResourceRef};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GmObject {
    #[serde(rename = "$GMObject")]
    pub gm_object: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    #[serde(rename = "eventList")]
    pub event_list: Vec<Event>,

    pub managed: bool,
    pub name: String,

    #[serde(rename = "overriddenProperties")]
    pub overridden_properties: Vec<serde_json::Value>,

    pub parent: ResourceRef,

    #[serde(rename = "parentObjectId")]
    pub parent_object_id: Option<ResourceRef>,

    pub persistent: bool,

    #[serde(rename = "physicsAngularDamping")]
    pub physics_angular_damping: f32,

    #[serde(rename = "physicsDensity")]
    pub physics_density: f32,

    #[serde(rename = "physicsFriction")]
    pub physics_friction: f32,

    #[serde(rename = "physicsGroup")]
    pub physics_group: i32,

    #[serde(rename = "physicsKinematic")]
    pub physics_kinematic: bool,

    #[serde(rename = "physicsLinearDamping")]
    pub physics_linear_damping: f32,

    #[serde(rename = "physicsObject")]
    pub physics_object: bool,

    #[serde(rename = "physicsRestitution")]
    pub physics_restitution: f32,

    #[serde(rename = "physicsSensor")]
    pub physics_sensor: bool,

    #[serde(rename = "physicsShape")]
    pub physics_shape: i32,

    #[serde(rename = "physicsShapePoints")]
    pub physics_shape_points: Vec<serde_json::Value>,

    #[serde(rename = "physicsStartAwake")]
    pub physics_start_awake: bool,

    pub properties: Vec<serde_json::Value>,

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    pub solid: bool,

    #[serde(rename = "spriteId")]
    pub sprite_id: Option<ResourceRef>,

    #[serde(rename = "spriteMaskId")]
    pub sprite_mask_id: Option<ResourceRef>,

    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Event {
    #[serde(rename = "$GMEvent")]
    pub gm_event: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    #[serde(rename = "collisionObjectId")]
    pub collision_object_id: Option<ResourceRef>,

    #[serde(rename = "eventNum")]
    pub event_num: i32,

    #[serde(rename = "eventType")]
    pub event_type: i32,

    #[serde(rename = "isDnD")]
    pub is_dnd: bool,

    pub name: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            gm_event: "v1".into(),
            display_name_internal: "".into(),
            collision_object_id: None,
            event_num: 0,
            event_type: 0,
            is_dnd: false,
            name: "".into(),
            resource_type: "GMEvent".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl Default for GmObject {
    fn default() -> Self {
        Self {
            gm_object: "".into(),
            display_name_internal: "Object1".into(),

            event_list: vec![Event::default()],

            managed: true,
            name: "Object1".into(),

            overridden_properties: Vec::new(),

            parent: ResourceRef {
                name: "BLANK GAME".into(),
                path: "BLANK GAME.yyp".into(),
            },

            parent_object_id: None,

            persistent: false,

            physics_angular_damping: 0.1,
            physics_density: 0.5,
            physics_friction: 0.2,
            physics_group: 1,
            physics_kinematic: false,
            physics_linear_damping: 0.1,
            physics_object: false,
            physics_restitution: 0.1,
            physics_sensor: false,
            physics_shape: 1,
            physics_shape_points: Vec::new(),
            physics_start_awake: true,

            properties: Vec::new(),

            resource_type: "GMObject".into(),
            resource_version: "2.0".into(),

            solid: false,

            sprite_id: None,
            sprite_mask_id: None,

            visible: true,
        }
    }
}

impl GmObject {
    pub fn new(name: &str, parent: ResourceRef) -> Self {
        Self {
            display_name_internal: name.to_string(),
            name: name.to_string(),
            parent,
            ..Default::default()
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        // read as json5
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let value: serde_json::Value = json5::from_str(&contents)?;
        let object: GmObject = serde_json::from_value(value)?;
        Ok(object)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;
        Ok(())
    }
}

impl Resource for GmObject {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_path(&self) -> &str {
        &self.parent.path
    }
}
