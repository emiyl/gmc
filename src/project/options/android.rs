use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AndroidOptions {
    #[serde(rename = "$GMAndroidOptions")]
    pub gm_android_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_android_application_tag_inject: String,
    pub option_android_arch_arm64: bool,
    pub option_android_arch_armv7: bool,
    pub option_android_arch_x86_64: bool,
    pub option_android_attribute_allow_backup: bool,
    pub option_android_build_tools: String,
    pub option_android_compile_sdk: String,
    pub option_android_device_support: i32,
    pub option_android_display_layout: String,
    pub option_android_display_name: String,
    pub option_android_edge_to_edge_display: bool,
    pub option_android_facebook_app_display_name: String,
    pub option_android_facebook_id: String,
    pub option_android_gamepad_support: bool,
    pub option_android_google_apk_expansion: bool,
    pub option_android_google_cloud_saving: bool,
    pub option_android_google_dynamic_asset_delivery: bool,
    pub option_android_google_licensing_public_key: String,
    pub option_android_google_services_app_id: String,
    pub option_android_gradle_plugin_version: String,
    pub option_android_gradle_version: String,

    pub option_android_icon_adaptivebg_hdpi: String,
    pub option_android_icon_adaptivebg_ldpi: String,
    pub option_android_icon_adaptivebg_mdpi: String,
    pub option_android_icon_adaptivebg_xhdpi: String,
    pub option_android_icon_adaptivebg_xxhdpi: String,
    pub option_android_icon_adaptivebg_xxxhdpi: String,

    pub option_android_icon_adaptive_generate: bool,

    pub option_android_icon_adaptive_hdpi: String,
    pub option_android_icon_adaptive_ldpi: String,
    pub option_android_icon_adaptive_mdpi: String,
    pub option_android_icon_adaptive_xhdpi: String,
    pub option_android_icon_adaptive_xxhdpi: String,
    pub option_android_icon_adaptive_xxxhdpi: String,

    pub option_android_icon_hdpi: String,
    pub option_android_icon_ldpi: String,
    pub option_android_icon_mdpi: String,
    pub option_android_icon_xhdpi: String,
    pub option_android_icon_xxhdpi: String,
    pub option_android_icon_xxxhdpi: String,

    pub option_android_install_location: i32,
    pub option_android_interpolate_pixels: bool,
    pub option_android_launchscreen_fill: i32,
    pub option_android_lint: bool,
    pub option_android_logcat: String,
    pub option_android_minimum_sdk: String,

    pub option_android_orient_landscape: bool,
    pub option_android_orient_landscape_flipped: bool,
    pub option_android_orient_portrait: bool,
    pub option_android_orient_portrait_flipped: bool,

    pub option_android_package_company: String,
    pub option_android_package_domain: String,
    pub option_android_package_product: String,

    pub option_android_permission_bluetooth: bool,
    pub option_android_permission_internet: bool,
    pub option_android_permission_network_state: bool,
    pub option_android_permission_read_phone_state: bool,
    pub option_android_permission_record_audio: bool,
    pub option_android_permission_write_external_storage: bool,

    pub option_android_proguard_minifying: bool,
    pub option_android_proguard_shrinking: bool,

    pub option_android_scale: i32,
    pub option_android_screen_depth: i32,
    pub option_android_sleep_margin: i32,
    pub option_android_splashscreen_background_colour: i32,

    pub option_android_splash_screens_landscape: String,
    pub option_android_splash_screens_portrait: String,

    pub option_android_splash_time: i32,

    pub option_android_support_lib: String,
    pub option_android_sync_amazon: bool,
    pub option_android_target_sdk: String,
    pub option_android_texture_page: String,
    pub option_android_tools_from_version: bool,

    pub option_android_tv_banner: String,
    pub option_android_tv_isgame: bool,
    pub option_android_tv_supports_leanback: bool,

    pub option_android_use_facebook: bool,
    pub option_android_version: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for AndroidOptions {
    fn default() -> Self {
        Self {
            gm_android_options: "v1".into(),
            display_name_internal: "Android".into(),
            name: "Android".into(),

            option_android_application_tag_inject: "".into(),
            option_android_arch_arm64: true,
            option_android_arch_armv7: false,
            option_android_arch_x86_64: false,
            option_android_attribute_allow_backup: false,
            option_android_build_tools: "".into(),
            option_android_compile_sdk: "".into(),
            option_android_device_support: 0,
            option_android_display_layout: "LAYOUT_IN_DISPLAY_CUTOUT_MODE_DEFAULT".into(),
            option_android_display_name: "test1".into(),
            option_android_edge_to_edge_display: false,
            option_android_facebook_app_display_name: "".into(),
            option_android_facebook_id: "".into(),
            option_android_gamepad_support: true,
            option_android_google_apk_expansion: false,
            option_android_google_cloud_saving: false,
            option_android_google_dynamic_asset_delivery: false,
            option_android_google_licensing_public_key: "".into(),
            option_android_google_services_app_id: "".into(),
            option_android_gradle_plugin_version: "8.8.0".into(),
            option_android_gradle_version: "8.10.2".into(),

            option_android_icon_adaptivebg_hdpi:
                "${base_options_dir}/android/icons_adaptivebg/hdpi.png".into(),
            option_android_icon_adaptivebg_ldpi:
                "${base_options_dir}/android/icons_adaptivebg/ldpi.png".into(),
            option_android_icon_adaptivebg_mdpi:
                "${base_options_dir}/android/icons_adaptivebg/mdpi.png".into(),
            option_android_icon_adaptivebg_xhdpi:
                "${base_options_dir}/android/icons_adaptivebg/xhdpi.png".into(),
            option_android_icon_adaptivebg_xxhdpi:
                "${base_options_dir}/android/icons_adaptivebg/xxhdpi.png".into(),
            option_android_icon_adaptivebg_xxxhdpi:
                "${base_options_dir}/android/icons_adaptivebg/xxxhdpi.png".into(),

            option_android_icon_adaptive_generate: false,

            option_android_icon_adaptive_hdpi:
                "${base_options_dir}/android/icons_adaptive/hdpi.png".into(),
            option_android_icon_adaptive_ldpi:
                "${base_options_dir}/android/icons_adaptive/ldpi.png".into(),
            option_android_icon_adaptive_mdpi:
                "${base_options_dir}/android/icons_adaptive/mdpi.png".into(),
            option_android_icon_adaptive_xhdpi:
                "${base_options_dir}/android/icons_adaptive/xhdpi.png".into(),
            option_android_icon_adaptive_xxhdpi:
                "${base_options_dir}/android/icons_adaptive/xxhdpi.png".into(),
            option_android_icon_adaptive_xxxhdpi:
                "${base_options_dir}/android/icons_adaptive/xxxhdpi.png".into(),

            option_android_icon_hdpi: "${base_options_dir}/android/icons/hdpi.png".into(),
            option_android_icon_ldpi: "${base_options_dir}/android/icons/ldpi.png".into(),
            option_android_icon_mdpi: "${base_options_dir}/android/icons/mdpi.png".into(),
            option_android_icon_xhdpi: "${base_options_dir}/android/icons/xhdpi.png".into(),
            option_android_icon_xxhdpi: "${base_options_dir}/android/icons/xxhdpi.png".into(),
            option_android_icon_xxxhdpi: "${base_options_dir}/android/icons/xxxhdpi.png".into(),

            option_android_install_location: 0,
            option_android_interpolate_pixels: true,
            option_android_launchscreen_fill: 0,
            option_android_lint: false,
            option_android_logcat: "yoyo:V DEBUG:V AndroidRuntime:V".into(),
            option_android_minimum_sdk: "".into(),

            option_android_orient_landscape: true,
            option_android_orient_landscape_flipped: true,
            option_android_orient_portrait: true,
            option_android_orient_portrait_flipped: true,

            option_android_package_company: "company".into(),
            option_android_package_domain: "com".into(),
            option_android_package_product: "game".into(),

            option_android_permission_bluetooth: true,
            option_android_permission_internet: true,
            option_android_permission_network_state: false,
            option_android_permission_read_phone_state: false,
            option_android_permission_record_audio: false,
            option_android_permission_write_external_storage: false,

            option_android_proguard_minifying: false,
            option_android_proguard_shrinking: false,

            option_android_scale: 0,
            option_android_screen_depth: 0,
            option_android_sleep_margin: 4,
            option_android_splashscreen_background_colour: 255,

            option_android_splash_screens_landscape:
                "${base_options_dir}/android/splash/landscape.png".into(),
            option_android_splash_screens_portrait:
                "${base_options_dir}/android/splash/portrait.png".into(),

            option_android_splash_time: 0,

            option_android_support_lib: "".into(),
            option_android_sync_amazon: false,
            option_android_target_sdk: "".into(),
            option_android_texture_page: "2048x2048".into(),
            option_android_tools_from_version: false,

            option_android_tv_banner: "${base_options_dir}/android/tv_banner.png".into(),
            option_android_tv_isgame: true,
            option_android_tv_supports_leanback: true,

            option_android_use_facebook: false,
            option_android_version: "1.0.0.0".into(),

            resource_type: "GMAndroidOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl AndroidOptions {
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
