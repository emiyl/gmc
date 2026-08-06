use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::project::resource::ResourceBase;

use super::ResourceId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GMRInstance {
    #[serde(rename = "$GMRInstance")]
    pub resource_tag: String,

    #[serde(flatten)]
    pub base: ResourceBase,

    pub colour: u32,

    pub frozen: bool,

    #[serde(rename = "hasCreationCode")]
    pub has_creation_code: bool,

    pub ignore: bool,

    #[serde(rename = "imageIndex")]
    pub image_index: i32,

    #[serde(rename = "imageSpeed")]
    pub image_speed: f32,

    #[serde(rename = "inheritCode")]
    pub inherit_code: bool,

    #[serde(rename = "inheritedItemId")]
    pub inherited_item_id: Option<String>,

    #[serde(rename = "inheritItemSettings")]
    pub inherit_item_settings: bool,

    #[serde(rename = "isDnd")]
    pub is_dnd: bool,

    #[serde(rename = "objectId")]
    pub object: Option<ResourceId>,

    pub properties: Vec<Value>,

    pub rotation: f32,

    #[serde(rename = "scaleX")]
    pub scale_x: f32,

    #[serde(rename = "scaleY")]
    pub scale_y: f32,

    pub x: f32,
    pub y: f32,
}

impl Default for GMRInstance {
    fn default() -> Self {
        let name = Self::new_instance_name();
        Self {
            resource_tag: "v4".to_string(),
            base: ResourceBase::new(&name, "GMRInstance"),

            colour: 0xFFFFFFFF,
            frozen: false,

            has_creation_code: false,
            ignore: false,

            image_index: 0,
            image_speed: 1.0,

            inherit_code: false,
            inherited_item_id: None,
            inherit_item_settings: false,
            is_dnd: false,

            object: None,

            properties: Vec::new(),

            rotation: 0.0,

            scale_x: 1.0,
            scale_y: 1.0,

            x: 0.0,
            y: 0.0,
        }
    }
}

impl GMRInstance {
    fn new_instance_name() -> String {
        // Generate a random 8-digit hexadecimal string to use as a unique instance name.
        let value: u32 = rand::rng().random();
        format!("inst_{:08X}", value)
    }

    pub fn new(object: ResourceId, x: f32, y: f32) -> Self {
        Self {
            object: Some(object),
            x,
            y,
            ..Default::default()
        }
    }
}
