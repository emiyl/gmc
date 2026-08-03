use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ResourceId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Instance {
    #[serde(rename = "$GMRInstance")]
    pub resource_tag: String,

    #[serde(rename = "%Name")]
    pub display_name: String,

    pub name: String,

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

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    pub rotation: f32,

    #[serde(rename = "scaleX")]
    pub scale_x: f32,

    #[serde(rename = "scaleY")]
    pub scale_y: f32,

    pub x: f32,
    pub y: f32,
}

impl Default for Instance {
    fn default() -> Self {
        let name = Self::new_instance_name();
        Self {
            resource_tag: "v4".to_string(),

            display_name: name.clone(),
            name: name.clone(),

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

            resource_type: "GMRInstance".into(),
            resource_version: "2.0".into(),

            rotation: 0.0,

            scale_x: 1.0,
            scale_y: 1.0,

            x: 0.0,
            y: 0.0,
        }
    }
}

impl Instance {
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
