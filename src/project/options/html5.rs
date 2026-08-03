use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::project::formatter::{format_gamemaker_json, read_gamemaker_json};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Html5Options {
    #[serde(rename = "$GMHtml5Options")]
    pub gm_html5_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_html5_allow_fullscreen: bool,
    pub option_html5_browser_title: String,
    pub option_html5_centregame: bool,
    pub option_html5_display_cursor: bool,
    pub option_html5_facebook_app_display_name: String,
    pub option_html5_facebook_id: String,
    pub option_html5_flurry_enable: bool,
    pub option_html5_flurry_id: String,
    pub option_html5_foldername: String,
    pub option_html5_google_analytics_enable: bool,
    pub option_html5_google_tracking_id: String,
    pub option_html5_icon: String,
    pub option_html5_index: String,
    pub option_html5_interpolate_pixels: bool,
    pub option_html5_jsprepend: String,
    pub option_html5_loadingbar: String,
    pub option_html5_localrunalert: bool,
    pub option_html5_outputdebugtoconsole: bool,
    pub option_html5_outputname: String,
    pub option_html5_scale: i32,
    pub option_html5_splash_png: String,
    pub option_html5_texture_page: String,
    pub option_html5_usebuiltinfont: bool,
    pub option_html5_usebuiltinparticles: bool,
    pub option_html5_usesplash: bool,
    pub option_html5_use_facebook: bool,
    pub option_html5_version: String,
    pub option_html5_webgl: i32,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for Html5Options {
    fn default() -> Self {
        Self {
            gm_html5_options: "".into(),
            display_name_internal: "HTML5".into(),
            name: "HTML5".into(),

            option_html5_allow_fullscreen: true,
            option_html5_browser_title: "test1".into(),
            option_html5_centregame: false,
            option_html5_display_cursor: true,
            option_html5_facebook_app_display_name: "".into(),
            option_html5_facebook_id: "".into(),
            option_html5_flurry_enable: false,
            option_html5_flurry_id: "".into(),
            option_html5_foldername: "html5game".into(),
            option_html5_google_analytics_enable: false,
            option_html5_google_tracking_id: "".into(),
            option_html5_icon: "${base_options_dir}/html5/fav.ico".into(),
            option_html5_index: "".into(),
            option_html5_interpolate_pixels: true,
            option_html5_jsprepend: "".into(),
            option_html5_loadingbar: "".into(),
            option_html5_localrunalert: true,
            option_html5_outputdebugtoconsole: true,
            option_html5_outputname: "index.html".into(),
            option_html5_scale: 0,
            option_html5_splash_png: "${base_options_dir}/html5/splash.png".into(),
            option_html5_texture_page: "2048x2048".into(),
            option_html5_usebuiltinfont: true,
            option_html5_usebuiltinparticles: true,
            option_html5_usesplash: false,
            option_html5_use_facebook: false,
            option_html5_version: "1.0.0.0".into(),
            option_html5_webgl: 2,

            resource_type: "GMHtml5Options".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl Html5Options {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let value = read_gamemaker_json(path)?;
        let options = serde_json::from_value(value).expect("Failed to deserialize Html5Options");
        Ok(options)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let value = serde_json::to_value(self)?;
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;
        Ok(())
    }
}
