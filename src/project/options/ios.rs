use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IosOptions {
    #[serde(rename = "$GMiOSOptions")]
    pub gm_ios_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_ios_build_number: i32,
    pub option_ios_bundle_name: String,
    pub option_ios_defer_home_indicator: bool,
    pub option_ios_devices: i32,
    pub option_ios_display_name: String,
    pub option_ios_enable_broadcast: bool,
    pub option_ios_half_ipad1_textures: bool,

    pub option_ios_icon_ipad_app_152: String,
    pub option_ios_icon_ipad_app_76: String,
    pub option_ios_icon_ipad_notification_20: String,
    pub option_ios_icon_ipad_notification_40: String,
    pub option_ios_icon_ipad_pro_app_167: String,
    pub option_ios_icon_ipad_settings_29: String,
    pub option_ios_icon_ipad_settings_58: String,
    pub option_ios_icon_ipad_spotlight_40: String,
    pub option_ios_icon_ipad_spotlight_80: String,

    pub option_ios_icon_iphone_app_120: String,
    pub option_ios_icon_iphone_app_180: String,
    pub option_ios_icon_iphone_notification_40: String,
    pub option_ios_icon_iphone_notification_60: String,
    pub option_ios_icon_iphone_settings_58: String,
    pub option_ios_icon_iphone_settings_87: String,
    pub option_ios_icon_iphone_spotlight_120: String,
    pub option_ios_icon_iphone_spotlight_80: String,

    pub option_ios_icon_itunes_artwork_1024: String,

    pub option_ios_interpolate_pixels: bool,
    pub option_ios_launchscreen_fill: i32,
    pub option_ios_launchscreen_image: String,
    pub option_ios_launchscreen_image_landscape: String,

    pub option_ios_min_version: String,

    pub option_ios_orientation_landscape: bool,
    pub option_ios_orientation_landscape_flipped: bool,
    pub option_ios_orientation_portrait: bool,
    pub option_ios_orientation_portrait_flipped: bool,

    pub option_ios_output_dir: String,
    pub option_ios_podfile_lock_path: String,
    pub option_ios_podfile_path: String,

    pub option_ios_scale: i32,
    pub option_ios_splashscreen_background_colour: u32,
    pub option_ios_team_id: String,
    pub option_ios_texture_page: String,
    pub option_ios_version: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for IosOptions {
    fn default() -> Self {
        Self {
            gm_ios_options: "v1".into(),
            display_name_internal: "iOS".into(),
            name: "iOS".into(),

            option_ios_build_number: 0,
            option_ios_bundle_name: "com.company.game".into(),
            option_ios_defer_home_indicator: false,
            option_ios_devices: 2,
            option_ios_display_name: "BLANK GAME".into(),
            option_ios_enable_broadcast: false,
            option_ios_half_ipad1_textures: false,

            option_ios_icon_ipad_app_152: "${base_options_dir}/ios/icons/app/ipad_152.png".into(),
            option_ios_icon_ipad_app_76: "${base_options_dir}/ios/icons/app/ipad_76.png".into(),
            option_ios_icon_ipad_notification_20:
                "${base_options_dir}/ios/icons/notification/ipad_20.png".into(),
            option_ios_icon_ipad_notification_40:
                "${base_options_dir}/ios/icons/notification/ipad_40.png".into(),
            option_ios_icon_ipad_pro_app_167: "${base_options_dir}/ios/icons/app/ipad_pro_167.png"
                .into(),
            option_ios_icon_ipad_settings_29: "${base_options_dir}/ios/icons/settings/ipad_29.png"
                .into(),
            option_ios_icon_ipad_settings_58: "${base_options_dir}/ios/icons/settings/ipad_58.png"
                .into(),
            option_ios_icon_ipad_spotlight_40:
                "${base_options_dir}/ios/icons/spotlight/ipad_40.png".into(),
            option_ios_icon_ipad_spotlight_80:
                "${base_options_dir}/ios/icons/spotlight/ipad_80.png".into(),

            option_ios_icon_iphone_app_120: "${base_options_dir}/ios/icons/app/iphone_120.png"
                .into(),
            option_ios_icon_iphone_app_180: "${base_options_dir}/ios/icons/app/iphone_180.png"
                .into(),
            option_ios_icon_iphone_notification_40:
                "${base_options_dir}/ios/icons/notification/iphone_40.png".into(),
            option_ios_icon_iphone_notification_60:
                "${base_options_dir}/ios/icons/notification/iphone_60.png".into(),
            option_ios_icon_iphone_settings_58:
                "${base_options_dir}/ios/icons/settings/iphone_58.png".into(),
            option_ios_icon_iphone_settings_87:
                "${base_options_dir}/ios/icons/settings/iphone_87.png".into(),
            option_ios_icon_iphone_spotlight_120:
                "${base_options_dir}/ios/icons/spotlight/iphone_120.png".into(),
            option_ios_icon_iphone_spotlight_80:
                "${base_options_dir}/ios/icons/spotlight/iphone_80.png".into(),

            option_ios_icon_itunes_artwork_1024:
                "${base_options_dir}/ios/icons/itunes/itunes_1024.png".into(),

            option_ios_interpolate_pixels: true,
            option_ios_launchscreen_fill: 0,
            option_ios_launchscreen_image: "${base_options_dir}/ios/splash/launchscreen.png".into(),
            option_ios_launchscreen_image_landscape:
                "${base_options_dir}/ios/splash/launchscreen-landscape.png".into(),

            option_ios_min_version: "10.0".into(),

            option_ios_orientation_landscape: true,
            option_ios_orientation_landscape_flipped: true,
            option_ios_orientation_portrait: true,
            option_ios_orientation_portrait_flipped: true,

            option_ios_output_dir: "~/gamemakerstudio2".into(),
            option_ios_podfile_lock_path: "${options_dir}/ios/Podfile.lock".into(),
            option_ios_podfile_path: "${options_dir}/ios/Podfile".into(),

            option_ios_scale: 0,
            option_ios_splashscreen_background_colour: 255,
            option_ios_team_id: "".into(),
            option_ios_texture_page: "2048x2048".into(),
            option_ios_version: "1.0.0.0".into(),

            resource_type: "GMiOSOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl IosOptions {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            option_ios_display_name: name.into(),
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
