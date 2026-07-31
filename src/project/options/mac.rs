use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MacOptions {
    #[serde(rename = "$GMMacOptions")]
    pub gm_mac_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_mac_allow_fullscreen: bool,
    pub option_mac_allow_incoming_network: bool,
    pub option_mac_allow_outgoing_network: bool,
    pub option_mac_apple_sign_in: bool,
    pub option_mac_app_category: String,
    pub option_mac_app_id: String,
    pub option_mac_arm64: bool,
    pub option_mac_build_app_store: bool,
    pub option_mac_build_number: i32,
    pub option_mac_copyright: String,
    pub option_mac_disable_sandbox: bool,
    pub option_mac_display_cursor: bool,
    pub option_mac_display_name: String,
    pub option_mac_enable_retina: bool,
    pub option_mac_enable_steam: bool,
    pub option_mac_icon_png: String,
    pub option_mac_installer_background_png: String,
    pub option_mac_interpolate_pixels: bool,
    pub option_mac_menu_dock: bool,
    pub option_mac_min_version: String,
    pub option_mac_output_dir: String,
    pub option_mac_resize_window: bool,
    pub option_mac_scale: i32,
    pub option_mac_signing_identity: String,
    pub option_mac_splash_png: String,
    pub option_mac_start_fullscreen: bool,
    pub option_mac_team_id: String,
    pub option_mac_texture_page: String,
    pub option_mac_version: String,
    pub option_mac_vsync: bool,
    pub option_mac_x86_64: bool,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for MacOptions {
    fn default() -> Self {
        Self {
            gm_mac_options: "".into(),
            display_name_internal: "macOS".into(),
            name: "macOS".into(),

            option_mac_allow_fullscreen: false,
            option_mac_allow_incoming_network: false,
            option_mac_allow_outgoing_network: false,
            option_mac_apple_sign_in: false,
            option_mac_app_category: "Games".into(),
            option_mac_app_id: "com.company.game".into(),
            option_mac_arm64: true,
            option_mac_build_app_store: false,
            option_mac_build_number: 0,
            option_mac_copyright: "".into(),
            option_mac_disable_sandbox: false,
            option_mac_display_cursor: true,
            option_mac_display_name: "BLANK GAME".into(),
            option_mac_enable_retina: false,
            option_mac_enable_steam: false,
            option_mac_icon_png: "${base_options_dir}/mac/icons/1024.png".into(),
            option_mac_installer_background_png:
                "${base_options_dir}/mac/splash/installer_background.png".into(),
            option_mac_interpolate_pixels: true,
            option_mac_menu_dock: false,
            option_mac_min_version: "10.10".into(),
            option_mac_output_dir: "~/gamemakerstudio2".into(),
            option_mac_resize_window: false,
            option_mac_scale: 0,
            option_mac_signing_identity: "Developer ID Application:".into(),
            option_mac_splash_png: "${base_options_dir}/mac/splash/splash.png".into(),
            option_mac_start_fullscreen: false,
            option_mac_team_id: "".into(),
            option_mac_texture_page: "2048x2048".into(),
            option_mac_version: "1.0.0.0".into(),
            option_mac_vsync: true,
            option_mac_x86_64: true,

            resource_type: "GMMacOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl MacOptions {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            option_mac_display_name: name.into(),
            ..Self::default()
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let text = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&text)?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}
