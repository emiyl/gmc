mod event;
use event::Event;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use super::{ResourceId, ResourceTrait};
use crate::project::{
    formatter::format_gamemaker_json,
    resource::{
        ResourceBase,
        object::event::{EventSubType, EventType},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GMObject {
    #[serde(rename = "$GMObject")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub base: ResourceBase,
    pub parent: ResourceId,

    #[serde(rename = "eventList")]
    pub event_list: Vec<Event>,

    pub managed: bool,

    #[serde(rename = "overriddenProperties")]
    pub overridden_properties: Vec<serde_json::Value>,

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

    pub solid: bool,

    #[serde(rename = "spriteId")]
    pub sprite_id: Option<ResourceId>,

    #[serde(rename = "spriteMaskId")]
    pub sprite_mask_id: Option<ResourceId>,

    pub visible: bool,
}

impl Default for GMObject {
    fn default() -> Self {
        Self {
            resource_tag: "".into(),
            base: ResourceBase::new("Object1", "GMObject"),
            parent: ResourceId::default(),

            event_list: vec![],

            managed: true,

            overridden_properties: Vec::new(),

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

            solid: false,

            sprite_id: None,
            sprite_mask_id: None,

            visible: true,
        }
    }
}

impl ResourceTrait for GMObject {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize GMObject");
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;

        for event in &self.event_list {
            event.ensure_code_file_exists(path)?;
        }

        Ok(())
    }

    fn default_path(&self) -> String {
        format!("objects/{}/{}.yy", self.base.name, self.base.name)
    }
}

impl GMObject {
    pub fn new(name: &str, parent: ResourceId) -> Self {
        Self {
            base: ResourceBase::new(name, "GMObject"),
            parent,
            ..Default::default()
        }
    }

    pub fn load(value: Value) -> std::io::Result<Self> {
        let object: GMObject =
            serde_json::from_value(value).expect("Failed to deserialize GMObject");
        Ok(object)
    }

    pub fn add_event(&mut self, event_type: String, event_subtype: Option<String>) {
        let event = Event::new(event_type, event_subtype);
        self.event_list.push(event);
    }

    pub fn get_event_code_list(
        &self,
        object_path: &std::path::Path,
    ) -> Vec<(EventType, EventSubType, String)> {
        let mut code_list = Vec::new();
        for event in &self.event_list {
            let event_type = event.get_event_type_enum();
            let event_subtype = event.get_event_subtype_enum();
            let code = event.get_code(object_path).unwrap_or_default();
            code_list.push((event_type, event_subtype, code));
        }
        code_list
    }
}
