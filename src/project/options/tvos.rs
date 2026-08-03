use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::project::formatter::{format_gamemaker_json, read_gamemaker_json};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TvOSOptions {
    #[serde(rename = "$GMtvOSOptions")]
    pub gm_tvos_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_tvos_build_number: i32,
    pub option_tvos_bundle_name: String,
    pub option_tvos_display_cursor: bool,
    pub option_tvos_display_name: String,
    pub option_tvos_enable_broadcast: bool,

    pub option_tvos_icon_1280: String,
    pub option_tvos_icon_400: String,
    pub option_tvos_icon_400_2x: String,

    pub option_tvos_interpolate_pixels: bool,
    pub option_tvos_min_version: String,
    pub option_tvos_output_dir: String,
    pub option_tvos_podfile_lock_path: String,
    pub option_tvos_podfile_path: String,
    pub option_tvos_scale: i32,

    pub option_tvos_splashscreen: String,
    pub option_tvos_splashscreen_2x: String,
    pub option_tvos_splash_time: i32,

    pub option_tvos_team_id: String,
    pub option_tvos_texture_page: String,

    pub option_tvos_topshelf: String,
    pub option_tvos_topshelf_2x: String,
    pub option_tvos_topshelf_wide: String,
    pub option_tvos_topshelf_wide_2x: String,

    pub option_tvos_version: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for TvOSOptions {
    fn default() -> Self {
        Self {
            gm_tvos_options: "v1".into(),
            display_name_internal: "tvOS".into(),
            name: "tvOS".into(),

            option_tvos_build_number: 0,
            option_tvos_bundle_name: "com.company.game".into(),
            option_tvos_display_cursor: false,
            option_tvos_display_name: "BLANK GAME".into(),
            option_tvos_enable_broadcast: false,

            option_tvos_icon_1280: "${base_options_dir}/tvos/icons/1280.png".into(),
            option_tvos_icon_400: "${base_options_dir}/tvos/icons/400.png".into(),
            option_tvos_icon_400_2x: "${base_options_dir}/tvos/icons/400_2x.png".into(),

            option_tvos_interpolate_pixels: true,
            option_tvos_min_version: "10.0".into(),
            option_tvos_output_dir: "~/GameMakerStudio2/tvOS".into(),
            option_tvos_podfile_lock_path: "${options_dir}\\tvos\\Podfile.lock".into(),
            option_tvos_podfile_path: "${options_dir}\\tvos\\Podfile".into(),
            option_tvos_scale: 0,

            option_tvos_splashscreen: "${base_options_dir}/tvos/splash/splash.png".into(),
            option_tvos_splashscreen_2x: "${base_options_dir}/tvos/splash/splash_2x.png".into(),
            option_tvos_splash_time: 0,

            option_tvos_team_id: "".into(),
            option_tvos_texture_page: "2048x2048".into(),

            option_tvos_topshelf: "${base_options_dir}/tvos/topshelf/topshelf.png".into(),
            option_tvos_topshelf_2x: "${base_options_dir}/tvos/topshelf/topshelf_2x.png".into(),
            option_tvos_topshelf_wide: "${base_options_dir}/tvos/topshelf/topshelf_wide.png".into(),
            option_tvos_topshelf_wide_2x: "${base_options_dir}/tvos/topshelf/topshelf_wide_2x.png"
                .into(),

            option_tvos_version: "1.0.0.0".into(),

            resource_type: "GMtvOSOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl TvOSOptions {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            option_tvos_display_name: name.into(),
            ..Self::default()
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let value = read_gamemaker_json(path)?;
        let options = serde_json::from_value(value).expect("Failed to deserialize TvOSOptions");
        Ok(options)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;
        Ok(())
    }
}
