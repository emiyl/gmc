use crate::project::{
    ResourceKind,
    formatter::{format_gamemaker_json, read_gamemaker_json},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub fn new(name: String, order: i32, path: String) -> Self {
        ResourceOrderSettingsItem { name, order, path }
    }
}

impl ResourceOrder {
    pub fn new() -> Self {
        ResourceOrder {
            folder_order_settings: Vec::new(),
            resource_order_settings: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, resource_kind: &ResourceKind, name: String, path: String) {
        if matches!(resource_kind, ResourceKind::Room) {
            return;
        }

        let last_order = self
            .resource_order_settings
            .iter()
            .map(|item| item.order)
            .max()
            .unwrap_or(0);

        self.resource_order_settings
            .push(ResourceOrderSettingsItem::new(name, last_order + 1, path));
    }

    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let value = read_gamemaker_json(path)?;
        let resource_order =
            serde_json::from_value(value).expect("Failed to deserialize ResourceOrder");
        Ok(resource_order)
    }
}
