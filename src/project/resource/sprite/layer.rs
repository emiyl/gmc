use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::project::resource::ResourceBase;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpriteImageLayer {
    Image(GMImageLayer),
    Folder(GMImageFolderLayer),
}

impl Default for SpriteImageLayer {
    fn default() -> Self {
        SpriteImageLayer::Image(GMImageLayer::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMImageLayerBase {
    #[serde(flatten)]
    pub base: ResourceBase,

    #[serde(rename = "blendMode")]
    pub blend_mode: i32,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "isLocked")]
    pub is_locked: bool,

    pub opacity: f32,
    pub visible: bool,
}

impl Default for GMImageLayerBase {
    fn default() -> Self {
        let uuid = Uuid::new_v4().to_string();
        Self {
            base: ResourceBase::new(uuid.as_str(), "GMImageLayer"),
            blend_mode: 0,
            display_name: "default".into(),
            is_locked: false,
            opacity: 100.0,
            visible: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMImageLayer {
    #[serde(rename = "$GMImageLayer")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub base: GMImageLayerBase,
}

impl Default for GMImageLayer {
    fn default() -> Self {
        Self {
            resource_tag: String::new(),
            base: GMImageLayerBase::default(),
        }
    }
}

impl GMImageLayer {
    pub fn ensure_image_exists(
        &self,
        frame_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get file name from path
        let stem = frame_path.file_stem().ok_or("No file stem")?;

        // create "layers" directory in the same directory as the frame_path
        let layers_dir = frame_path.with_file_name("layers");
        if !layers_dir.exists() {
            std::fs::create_dir_all(&layers_dir)?;
        }

        // create directory in the same directory as the frame_path with the name of the file without extension
        let dir_path = layers_dir.join(stem);
        if !dir_path.exists() {
            std::fs::create_dir_all(&dir_path)?;
        }

        // create blank 64x64 png image and save it to the layer's path
        let img_path = dir_path.join(format!("{}.png", self.base.base.name));
        if !img_path.exists() {
            let img = image::ImageBuffer::from_pixel(64, 64, image::Rgba([0u8, 0, 0, 0]));
            img.save(&img_path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMImageFolderLayer {
    #[serde(rename = "$GMImageFolderLayer")]
    pub resource_tag: String,

    #[serde(flatten)]
    pub base: GMImageLayerBase,

    pub layers: Vec<GMImageLayer>,
}

impl Default for GMImageFolderLayer {
    fn default() -> Self {
        Self {
            resource_tag: String::new(),
            base: GMImageLayerBase::default(),
            layers: Vec::new(),
        }
    }
}

impl GMImageFolderLayer {
    pub fn add_layer(&mut self, layer: GMImageLayer) {
        self.layers.push(layer);
    }

    pub fn remove_layer(&mut self, index: usize) -> Option<GMImageLayer> {
        if index < self.layers.len() {
            Some(self.layers.remove(index))
        } else {
            None
        }
    }
}
