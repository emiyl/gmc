use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::project::{ResourceId, ResourceTrait, resource::ResourceBase};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GMPath {
    #[serde(rename = "$GMPath", default)]
    pub resource_tag: String,
    #[serde(flatten)]
    pub base: ResourceBase,

    pub closed: bool,
    pub kind: i32,
    pub parent: ResourceId,
    pub points: Vec<GMPathPoint>,
    pub precision: i32,
}

impl Default for GMPath {
    fn default() -> Self {
        Self {
            resource_tag: String::new(),
            base: ResourceBase::new("Path1", "GMPath"),
            closed: false,
            kind: 0,
            parent: ResourceId::default(),
            points: Vec::new(),
            precision: 4,
        }
    }
}

impl ResourceTrait for GMPath {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize Path");
        let json = crate::project::formatter::format_gamemaker_json(&value);
        std::fs::write(path, json)?;
        Ok(())
    }

    fn default_path(&self) -> String {
        format!("paths/{}/{}.yy", self.base.name, self.base.name)
    }
}

impl GMPath {
    pub fn new(name: &str, parent: ResourceId) -> Self {
        Self {
            base: ResourceBase::new(&name, "GMPath"),
            parent: parent,
            ..Default::default()
        }
    }

    pub fn load(value: Value) -> std::io::Result<Self> {
        let path: GMPath = serde_json::from_value(value).expect("Failed to deserialize Path");
        Ok(path)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GMPathPoint {
    pub speed: f32,
    pub x: f32,
    pub y: f32,
}

impl GMPathPoint {
    pub fn new(x: f32, y: f32, speed: f32) -> Self {
        Self { x, y, speed }
    }
}
