use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::project::formatter::{format_gamemaker_json, read_gamemaker_json};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OperaGXOptions {
    #[serde(rename = "$GMOperaGXOptions")]
    pub gm_operagx_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_operagx_display_cursor: bool,
    #[serde(rename = "option_operagx_editUrl")]
    pub option_operagx_edit_url: String,
    pub option_operagx_game_name: String,
    pub option_operagx_guid: String,
    #[serde(rename = "option_operagx_internalShareUrl")]
    pub option_operagx_internal_share_url: String,
    pub option_operagx_interpolate_pixels: bool,

    #[serde(rename = "option_operagx_mod_editUrl")]
    pub option_operagx_mod_edit_url: String,
    pub option_operagx_mod_game_name: String,
    pub option_operagx_mod_guid: String,
    #[serde(rename = "option_operagx_mod_internalShareUrl")]
    pub option_operagx_mod_internal_share_url: String,
    pub option_operagx_mod_next_version: String,
    #[serde(rename = "option_operagx_mod_publicShareUrl")]
    pub option_operagx_mod_public_share_url: String,
    pub option_operagx_mod_team_id: String,
    pub option_operagx_mod_team_name: String,
    pub option_operagx_mod_version: String,

    pub option_operagx_next_version: String,
    #[serde(rename = "option_operagx_publicShareUrl")]
    pub option_operagx_public_share_url: String,
    pub option_operagx_scale: i32,
    pub option_operagx_team_id: String,
    pub option_operagx_team_name: String,
    pub option_operagx_texture_page: String,
    pub option_operagx_transparent_background: bool,
    pub option_operagx_version: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for OperaGXOptions {
    fn default() -> Self {
        Self {
            gm_operagx_options: "v1".into(),
            display_name_internal: "Opera GX".into(),
            name: "Opera GX".into(),

            option_operagx_display_cursor: true,
            option_operagx_edit_url: "".into(),
            option_operagx_game_name: "new_blank".into(),
            option_operagx_guid: "".into(),
            option_operagx_internal_share_url: "".into(),
            option_operagx_interpolate_pixels: true,

            option_operagx_mod_edit_url: "".into(),
            option_operagx_mod_game_name: "new_blank".into(),
            option_operagx_mod_guid: "".into(),
            option_operagx_mod_internal_share_url: "".into(),
            option_operagx_mod_next_version: "1.0.0.0".into(),
            option_operagx_mod_public_share_url: "".into(),
            option_operagx_mod_team_id: "".into(),
            option_operagx_mod_team_name: "".into(),
            option_operagx_mod_version: "1.0.0.0".into(),

            option_operagx_next_version: "1.0.0.0".into(),
            option_operagx_public_share_url: "".into(),
            option_operagx_scale: 0,
            option_operagx_team_id: "".into(),
            option_operagx_team_name: "".into(),
            option_operagx_texture_page: "2048x2048".into(),
            option_operagx_transparent_background: false,
            option_operagx_version: "1.0.0.0".into(),

            resource_type: "GMOperaGXOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl OperaGXOptions {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let value = read_gamemaker_json(path)?;
        let options = serde_json::from_value(value).expect("Failed to deserialize OperaGXOptions");
        Ok(options)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;
        Ok(())
    }
}
