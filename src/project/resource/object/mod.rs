mod event;
use event::Event;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use super::{ResourceId, ResourceTrait};
use crate::project::formatter::format_gamemaker_json;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Object {
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

    pub parent: ResourceId,

    #[serde(rename = "parentObjectId")]
    pub parent_object_id: Option<ResourceId>,

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
    pub sprite_id: Option<ResourceId>,

    #[serde(rename = "spriteMaskId")]
    pub sprite_mask_id: Option<ResourceId>,

    pub visible: bool,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            gm_object: "".into(),
            display_name_internal: "Object1".into(),

            event_list: vec![],

            managed: true,
            name: "Object1".into(),

            overridden_properties: Vec::new(),

            parent: ResourceId {
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

impl ResourceTrait for Object {
    fn name(&self) -> &str {
        &self.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize Object");
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;

        for event in &self.event_list {
            event.ensure_code_file_exists(path)?;
        }

        Ok(())
    }

    fn default_path(&self) -> String {
        format!("objects/{}/{}.yy", self.name, self.name)
    }
}

impl Object {
    pub fn new(name: &str, parent: ResourceId) -> Self {
        Self {
            display_name_internal: name.to_string(),
            name: name.to_string(),
            parent,
            ..Default::default()
        }
    }

    pub fn load(value: Value) -> std::io::Result<Self> {
        let object: Object = serde_json::from_value(value).expect("Failed to deserialize Object");
        Ok(object)
    }

    pub fn add_event(&mut self, event_type: String, event_subtype: Option<String>) {
        let event = Event::new(event_type, event_subtype);
        self.event_list.push(event);
    }
}
