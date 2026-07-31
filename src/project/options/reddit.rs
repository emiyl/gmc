use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditOptions {
    #[serde(rename = "$GMRedditOptions")]
    pub gm_reddit_options: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    pub name: String,

    pub option_reddit_devvit_project_id: String,
    pub option_reddit_devvit_project_path: String,
    pub option_reddit_display_cursor: bool,
    pub option_reddit_game_name: String,
    pub option_reddit_interpolate_pixels: bool,
    pub option_reddit_scale: i32,
    pub option_reddit_texture_page: String,
    pub option_reddit_transparent_background: bool,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for RedditOptions {
    fn default() -> Self {
        Self {
            gm_reddit_options: "v2".into(),
            display_name_internal: "Reddit".into(),
            name: "Reddit".into(),

            option_reddit_devvit_project_id: "new_blank".into(),
            option_reddit_devvit_project_path: "".into(),
            option_reddit_display_cursor: true,
            option_reddit_game_name: "new_blank".into(),
            option_reddit_interpolate_pixels: true,
            option_reddit_scale: 0,
            option_reddit_texture_page: "2048x2048".into(),
            option_reddit_transparent_background: false,

            resource_type: "GMRedditOptions".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl RedditOptions {
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
