use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::project::{formatter::format_gamemaker_json, resources::ResourceType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GmProjectYyp {
    #[serde(rename = "$GMProject")]
    pub gm_project: String,

    #[serde(rename = "%Name")]
    pub percent_name: String,

    #[serde(rename = "AudioGroups")]
    pub audio_groups: Vec<AudioGroups>,

    pub configs: Configs,

    #[serde(rename = "defaultScriptType")]
    pub default_script_type: i32,

    #[serde(rename = "Folders")]
    pub folders: Vec<Value>,

    #[serde(rename = "ForcedPrefabProjectReferences")]
    pub forced_prefab_project_references: Vec<Value>,

    #[serde(rename = "IncludedFiles")]
    pub included_files: Vec<Value>,

    #[serde(rename = "isEcma")]
    pub is_ecma: bool,

    #[serde(rename = "LibraryEmitters")]
    pub library_emitters: Vec<Value>,

    #[serde(rename = "MetaData")]
    pub meta_data: MetaData,

    pub name: String,

    pub resources: Vec<Resource>,

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    #[serde(rename = "RoomOrderNodes")]
    pub room_order_nodes: Vec<RoomOrderNode>,

    #[serde(rename = "templateType")]
    pub template_type: String,

    #[serde(rename = "TextureGroups")]
    pub texture_groups: Vec<TextureGroup>,
}

impl Default for GmProjectYyp {
    fn default() -> Self {
        GmProjectYyp {
            gm_project: "v1".to_string(),
            percent_name: "BLANK GAME".to_string(),
            audio_groups: vec![AudioGroups::default()],
            configs: Configs::default(),
            default_script_type: 0,
            folders: Vec::new(),
            forced_prefab_project_references: Vec::new(),
            included_files: Vec::new(),
            is_ecma: false,
            library_emitters: Vec::new(),
            meta_data: MetaData::default(),
            name: "BLANK PROJECT".to_string(),
            resources: Vec::new(),
            resource_type: "GMProject".to_string(),
            resource_version: "2.0".to_string(),
            room_order_nodes: Vec::new(),
            template_type: "game".to_string(),
            texture_groups: vec![TextureGroup::default()],
        }
    }
}

impl GmProjectYyp {
    pub fn new(name: &str) -> Self {
        GmProjectYyp {
            name: name.to_string(),
            percent_name: name.to_string(),
            ..GmProjectYyp::default()
        }
    }

    pub fn add_resource(&mut self, resource_type: ResourceType, name: &str, path: &str) {
        let resource_id = ResourceId {
            name: name.to_string(),
            path: path.to_string(),
        };

        let resource = Resource {
            id: resource_id.clone(),
        };
        self.resources.push(resource);

        if resource_type == ResourceType::Room {
            let room_order_node = RoomOrderNode {
                room_id: resource_id,
            };
            self.room_order_nodes.push(room_order_node);
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let project: GmProjectYyp = serde_json::from_reader(reader)?;
        Ok(project)
    }

    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        std::fs::write(path, json)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioGroups {
    #[serde(rename = "$GMAudioGroup")]
    pub gm_audio_group: String,
    #[serde(rename = "%Name")]
    pub percent_name: String,
    #[serde(rename = "exportDir")]
    pub export_dir: String,
    pub name: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    pub targets: i32,
}

impl Default for AudioGroups {
    fn default() -> Self {
        AudioGroups {
            gm_audio_group: "v1".to_string(),
            percent_name: "audiogroup_default".to_string(),
            export_dir: "".to_string(),
            name: "audiogroup_default".to_string(),
            resource_type: "GMAudioGroup".to_string(),
            resource_version: "2.0".to_string(),
            targets: -1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Configs {
    pub children: Vec<Value>,
    pub name: String,
}

impl Default for Configs {
    fn default() -> Self {
        Configs {
            children: Vec::new(),
            name: "Default".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(rename = "IDEVersion")]
    pub ide_version: String,
}

impl Default for MetaData {
    fn default() -> Self {
        MetaData {
            ide_version: "2026.0.0.16".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub id: ResourceId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomOrderNode {
    #[serde(rename = "roomId")]
    pub room_id: ResourceId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextureGroup {
    #[serde(rename = "$GMTextureGroup")]
    pub gm_texture_group: String,

    #[serde(rename = "%Name")]
    pub percent_name: String,

    pub autocrop: bool,

    pub border: i32,

    #[serde(rename = "compressFormat")]
    pub compress_format: String,

    #[serde(rename = "customOptions")]
    pub custom_options: String,

    pub directory: String,

    #[serde(rename = "groupParent")]
    pub group_parent: Value,

    #[serde(rename = "isScaled")]
    pub is_scaled: bool,

    #[serde(rename = "loadType")]
    pub load_type: String,

    #[serde(rename = "mipsToGenerate")]
    pub mips_to_generate: i32,

    pub name: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    pub targets: i32,
}

impl Default for TextureGroup {
    fn default() -> Self {
        TextureGroup {
            gm_texture_group: String::new(),
            percent_name: "Default".to_string(),
            autocrop: true,
            border: 2,
            compress_format: "bz2".to_string(),
            custom_options: String::new(),
            directory: String::new(),
            group_parent: Value::Null,
            is_scaled: true,
            load_type: "default".to_string(),
            mips_to_generate: 0,
            name: "Default".to_string(),
            resource_type: "GMTextureGroup".to_string(),
            resource_version: "2.0".to_string(),
            targets: -1,
        }
    }
}
