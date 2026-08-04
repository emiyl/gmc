use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::project::{ResourceId, ResourceTrait, formatter::format_gamemaker_json};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Script {
    #[serde(rename = "$GMScript")]
    gm_script: String,
    #[serde(rename = "%Name")]
    display_name_internal: String,
    #[serde(rename = "isCompatibility")]
    is_compatibility: bool,
    #[serde(rename = "isDnd")]
    is_dnd: bool,
    name: String,
    parent: ResourceId,
    #[serde(rename = "resourceType")]
    resource_type: String,
    #[serde(rename = "resourceVersion")]
    resource_version: String,
}

impl Default for Script {
    fn default() -> Self {
        Self {
            gm_script: "v1".into(),
            display_name_internal: "Script1".into(),
            is_compatibility: false,
            is_dnd: false,
            name: "Script1".into(),
            parent: ResourceId {
                name: "".into(),
                path: "".into(),
            },
            resource_type: "GMScript".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl ResourceTrait for Script {
    fn name(&self) -> &str {
        &self.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize Script");
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;

        self.ensure_code_file_exists(path)?;

        Ok(())
    }

    fn default_path(&self) -> String {
        format!("scripts/{}/{}.yy", self.name, self.name)
    }
}

impl Script {
    pub fn new(name: &str, parent: ResourceId) -> Self {
        Self {
            display_name_internal: name.into(),
            name: name.into(),
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
        script_path.with_file_name(format!("{}.gml", self.name))
    }

    pub fn ensure_code_file_exists(&self, path: &std::path::Path) -> std::io::Result<()> {
        let path = self.get_code_path_from_script_path(path);
        if !path.exists() {
            let mut file = fs::File::create(path)?;
            use std::io::Write;
            writeln!(file, "function {}(){{\n\n}}", self.name)?;
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
