use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::project::formatter::{format_gamemaker_json, read_gamemaker_json};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MainOptions {
    #[serde(rename = "$GMMainOptions")]
    pub gm_main_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_allow_instance_change: bool,
    pub option_audio_error_behaviour: bool,
    pub option_author: String,
    pub option_collision_compatibility: bool,
    pub option_copy_on_write_enabled: bool,
    pub option_draw_colour: u32,
    pub option_gameguid: String,
    pub option_gameid: String,
    pub option_game_speed: i32,
    pub option_legacy_json_parsing: bool,
    pub option_legacy_number_conversion: bool,
    pub option_legacy_other_behaviour: bool,
    pub option_legacy_primitive_drawing: bool,
    pub option_mips_for_3d_textures: bool,
    pub option_remove_unused_assets: bool,
    pub option_sci_usesci: bool,
    pub option_spine_licence: bool,
    pub option_steam_app_id: String,

    pub option_template_description: Option<String>,
    pub option_template_icon: String,
    pub option_template_image: String,

    pub option_window_colour: u32,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for MainOptions {
    fn default() -> Self {
        Self {
            gm_main_options: "v5".into(),
            display_name_internal: "Main".into(),
            name: "Main".into(),

            option_allow_instance_change: false,
            option_audio_error_behaviour: false,
            option_author: "".into(),
            option_collision_compatibility: false,
            option_copy_on_write_enabled: false,
            option_draw_colour: 0xFFFFFFFF,
            option_gameguid: Uuid::new_v4().to_string(),
            option_gameid: "0".into(),
            option_game_speed: 60,
            option_legacy_json_parsing: false,
            option_legacy_number_conversion: false,
            option_legacy_other_behaviour: false,
            option_legacy_primitive_drawing: false,
            option_mips_for_3d_textures: false,
            option_remove_unused_assets: true,
            option_sci_usesci: false,
            option_spine_licence: false,
            option_steam_app_id: "0".into(),

            option_template_description: None,
            option_template_icon: "${base_options_dir}/main/template_icon.png".into(),
            option_template_image: "${base_options_dir}/main/template_image.png".into(),

            option_window_colour: 255,

            resource_type: "GMMainOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl MainOptions {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            display_name_internal: name.into(),
            ..Self::default()
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let value = read_gamemaker_json(path)?;
        let options = serde_json::from_value(value).expect("Failed to deserialize MainOptions");
        Ok(options)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;
        Ok(())
    }
}
