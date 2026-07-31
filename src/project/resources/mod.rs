mod gm_object;
mod gm_room;

pub use gm_object::GmObject;
pub use gm_room::GmRoom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Object,
    Room,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ResourceRef {
    pub name: String,
    pub path: String,
}

impl Default for ResourceRef {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
        }
    }
}

impl ResourceRef {
    pub fn new(name: &str, path: &str) -> Self {
        ResourceRef {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

pub trait Resource {
    fn get_name(&self) -> &str;
    fn get_path(&self) -> &str;
}
