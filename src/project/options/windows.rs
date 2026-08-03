use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WindowsOptions {
    #[serde(rename = "$GMWindowsOptions")]
    pub gm_windows_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_windows_allow_fullscreen_switching: bool,
    pub option_windows_borderless: bool,
    pub option_windows_company_info: String,
    pub option_windows_copyright_info: String,
    pub option_windows_copy_exe_to_dest: bool,
    pub option_windows_d3dswapeffectdiscard: bool,
    pub option_windows_description_info: String,
    pub option_windows_disable_sandbox: bool,
    pub option_windows_display_cursor: bool,
    pub option_windows_display_name: String,
    pub option_windows_enable_steam: bool,
    pub option_windows_executable_name: String,
    pub option_windows_icon: String,
    pub option_windows_installer_finished: String,
    pub option_windows_installer_header: String,
    pub option_windows_interpolate_pixels: bool,
    pub option_windows_license: String,
    pub option_windows_nsis_file: String,
    pub option_windows_product_info: String,
    pub option_windows_resize_window: bool,
    pub option_windows_save_location: i32,
    pub option_windows_scale: i32,
    pub option_windows_sleep_margin: i32,
    pub option_windows_splash_screen: String,
    pub option_windows_start_fullscreen: bool,
    pub option_windows_steam_use_alternative_launcher: bool,
    pub option_windows_texture_page: String,
    pub option_windows_use_raw_mouse: bool,
    pub option_windows_use_splash: bool,
    pub option_windows_version: String,
    pub option_windows_vsync: bool,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for WindowsOptions {
    fn default() -> Self {
        Self {
            gm_windows_options: "v2".into(),
            display_name_internal: "Windows".into(),
            name: "Windows".into(),

            option_windows_allow_fullscreen_switching: false,
            option_windows_borderless: false,
            option_windows_company_info: "GMC".into(),
            option_windows_copyright_info: "".into(),
            option_windows_copy_exe_to_dest: false,
            option_windows_d3dswapeffectdiscard: false,
            option_windows_description_info: "A GMC Game".into(),
            option_windows_disable_sandbox: false,
            option_windows_display_cursor: true,
            option_windows_display_name: "BLANK GAME".into(),
            option_windows_enable_steam: false,
            option_windows_executable_name: "new_blank.exe".into(),
            option_windows_icon: "${base_options_dir}/windows/icons/icon.ico".into(),
            option_windows_installer_finished: "${base_options_dir}/windows/installer/finished.bmp"
                .into(),
            option_windows_installer_header: "${base_options_dir}/windows/installer/header.bmp"
                .into(),
            option_windows_interpolate_pixels: true,
            option_windows_license: "${base_options_dir}/windows/installer/license.txt".into(),
            option_windows_nsis_file: "${base_options_dir}/windows/installer/nsis_script.nsi"
                .into(),
            option_windows_product_info: "new_blank".into(),
            option_windows_resize_window: false,
            option_windows_save_location: 0,
            option_windows_scale: 0,
            option_windows_sleep_margin: 10,
            option_windows_splash_screen: "${base_options_dir}/windows/splash/splash.png".into(),
            option_windows_start_fullscreen: false,
            option_windows_steam_use_alternative_launcher: false,
            option_windows_texture_page: "2048x2048".into(),
            option_windows_use_raw_mouse: false,
            option_windows_use_splash: false,
            option_windows_version: "1.0.0.0".into(),
            option_windows_vsync: true,

            resource_type: "GMWindowsOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl WindowsOptions {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            display_name_internal: name.into(),
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
