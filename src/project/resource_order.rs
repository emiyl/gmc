use crate::project::formatter::format_gamemaker_json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Read;

use super::resources::ResourceType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceOrder {
    #[serde(rename = "FolderOrderSettings")]
    pub folder_order_settings: Vec<Value>,
    #[serde(rename = "ResourceOrderSettings")]
    pub resource_order_settings: Vec<ResourceOrderSettingsItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceOrderSettingsItem {
    pub name: String,
    pub order: i32,
    pub path: String,
}

impl Default for ResourceOrder {
    fn default() -> Self {
        ResourceOrder {
            folder_order_settings: Vec::new(),
            resource_order_settings: Vec::new(),
        }
    }
}

impl ResourceOrderSettingsItem {
    pub fn new(name: String, order: i32, resource_type: ResourceType) -> Self {
        ResourceOrderSettingsItem {
            name: name.clone(),
            order,
            path: match resource_type {
                ResourceType::Object => format!("objects/{}/{}.yy", name, name),
                _ => "".to_string(),
            },
        }
    }
}

impl ResourceOrder {
    pub fn new() -> Self {
        ResourceOrder {
            folder_order_settings: Vec::new(),
            resource_order_settings: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, name: String, resource_type: ResourceType) {
        if resource_type == ResourceType::Room {
            // For rooms, we don't add them to the resource order settings
            return;
        }

        let last_order = self
            .resource_order_settings
            .iter()
            .map(|item| item.order)
            .max()
            .unwrap_or(0);

        let new_item = ResourceOrderSettingsItem::new(name, last_order + 1, resource_type);
        self.resource_order_settings.push(new_item);
    }

    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let value: Value = json5::from_str(&contents).expect("Failed to parse JSON5");
        let resource_order: ResourceOrder =
            serde_json::from_value(value).expect("Failed to deserialize ResourceOrder");
        Ok(resource_order)
    }
}
