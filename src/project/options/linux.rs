use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::project::formatter::{format_gamemaker_json, read_gamemaker_json};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LinuxOptions {
    #[serde(rename = "$GMLinuxOptions")]
    pub gm_linux_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_linux_allow_fullscreen: bool,
    pub option_linux_disable_sandbox: bool,
    pub option_linux_display_cursor: bool,
    pub option_linux_display_name: String,
    pub option_linux_display_splash: bool,
    pub option_linux_enable_steam: bool,
    pub option_linux_homepage: String,
    pub option_linux_icon: String,
    pub option_linux_interpolate_pixels: bool,
    pub option_linux_long_desc: String,
    pub option_linux_maintainer_email: String,
    pub option_linux_resize_window: bool,
    pub option_linux_scale: i32,
    pub option_linux_short_desc: String,
    pub option_linux_splash_screen: String,
    pub option_linux_start_fullscreen: bool,
    pub option_linux_sync: bool,
    pub option_linux_texture_page: String,
    pub option_linux_version: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for LinuxOptions {
    fn default() -> Self {
        Self {
            gm_linux_options: "".into(),
            display_name_internal: "Linux".into(),
            name: "Linux".into(),

            option_linux_allow_fullscreen: false,
            option_linux_disable_sandbox: false,
            option_linux_display_cursor: true,
            option_linux_display_name: "BLANK GAME".into(),
            option_linux_display_splash: false,
            option_linux_enable_steam: false,
            option_linux_homepage: "http://www.yoyogames.com".into(),
            option_linux_icon: "${base_options_dir}/linux/icons/64.png".into(),
            option_linux_interpolate_pixels: true,
            option_linux_long_desc: "".into(),
            option_linux_maintainer_email: "".into(),
            option_linux_resize_window: false,
            option_linux_scale: 0,
            option_linux_short_desc: "".into(),
            option_linux_splash_screen: "${base_options_dir}/linux/splash/splash.png".into(),
            option_linux_start_fullscreen: false,
            option_linux_sync: true,
            option_linux_texture_page: "2048x2048".into(),
            option_linux_version: "1.0.0.0".into(),

            resource_type: "GMLinuxOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl LinuxOptions {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            option_linux_display_name: name.into(),
            ..Self::default()
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let value = read_gamemaker_json(path)?;
        let options = serde_json::from_value(value).expect("Failed to deserialize LinuxOptions");
        Ok(options)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;
        Ok(())
    }
}
