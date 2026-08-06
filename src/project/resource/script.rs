use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::project::{
    ResourceId, ResourceTrait, formatter::format_gamemaker_json, resource::ResourceBase,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GMScript {
    #[serde(rename = "$GMScript")]
    resource_tag: String,
    #[serde(flatten)]
    pub base: ResourceBase,
    parent: ResourceId,
    #[serde(rename = "isCompatibility")]
    is_compatibility: bool,
    #[serde(rename = "isDnd")]
    is_dnd: bool,
}

impl Default for GMScript {
    fn default() -> Self {
        Self {
            resource_tag: "v1".into(),
            base: ResourceBase::new("Script1", "GMScript"),
            is_compatibility: false,
            is_dnd: false,
            parent: ResourceId::default(),
        }
    }
}

impl ResourceTrait for GMScript {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize Script");
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;

        self.ensure_code_file_exists(path)?;

        Ok(())
    }

    fn default_path(&self) -> String {
        format!("scripts/{}/{}.yy", self.base.name, self.base.name)
    }
}

impl GMScript {
    pub fn new(name: &str, parent: ResourceId) -> Self {
        Self {
            base: ResourceBase::new(name, "GMScript"),
            parent,
            ..Default::default()
        }
    }

    pub fn load(value: Value) -> std::io::Result<Self> {
        let script = serde_json::from_value(value).expect("Failed to deserialize Script");
        Ok(script)
    }

    pub fn get_code_path_from_script_path(
        &self,
        script_path: &std::path::Path,
    ) -> std::path::PathBuf {
        // script_path is path to the script resource, e.g. "scripts/Script1/Script1.yy"
        // we need scripts/Script1/Script1.gml
        script_path.with_file_name(format!("{}.gml", self.base.name))
    }

    pub fn ensure_code_file_exists(&self, path: &std::path::Path) -> std::io::Result<()> {
        let path = self.get_code_path_from_script_path(path);
        if !path.exists() {
            let mut file = fs::File::create(path)?;
            use std::io::Write;
            writeln!(file, "function {}(){{\n\n}}", self.base.name)?;
        }
        Ok(())
    }

    pub fn get_code(&self, script_path: &std::path::Path) -> std::io::Result<String> {
        let path = self.get_code_path_from_script_path(script_path);
        if path.exists() {
            std::fs::read_to_string(path)
        } else {
            Ok(String::new())
        }
    }
}
