// Sprite
// SpriteFrame
// ImageLayer
// Sequence
// MessageEventStore
// MomentsEventStore
// SpriteFrameStore
// SpriteFrameKeyframe
// SpriteFrameChannel

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs};
use uuid::Uuid;

use crate::project::{
    ResourceId, ResourceTrait,
    formatter::format_gamemaker_json,
    resource::{ResourceBase, ResourceType},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMSprite {
    #[serde(rename = "$GMSprite")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub base: ResourceBase,
    pub parent: ResourceId,

    pub frames: Vec<GMSpriteFrame>,
    pub layers: Vec<GMImageLayer>,
    pub sequence: GMSequence,

    #[serde(rename = "bboxMode")]
    pub bbox_mode: i32,
    pub bbox_bottom: i32,
    pub bbox_left: i32,
    pub bbox_right: i32,
    pub bbox_top: i32,

    #[serde(rename = "collisionKind")]
    pub collision_kind: i32,
    #[serde(rename = "collisionTolerance")]
    pub collision_tolerance: i32,

    #[serde(rename = "DynamicTexturePage")]
    pub dynamic_texture_page: bool,
    #[serde(rename = "edgeFiltering")]
    pub edge_filtering: bool,
    #[serde(rename = "For3D")]
    pub for_3d: bool,

    #[serde(rename = "gridX")]
    pub grid_x: i32,
    #[serde(rename = "gridY")]
    pub grid_y: i32,

    pub height: i32,

    #[serde(rename = "HTile")]
    pub h_tile: bool,

    #[serde(rename = "nineSlice")]
    pub nine_slice: Value,

    pub origin: i32,

    #[serde(rename = "preMultiplyAlpha")]
    pub pre_multiply_alpha: bool,

    #[serde(rename = "swatchColours")]
    pub swatch_colours: Value,

    #[serde(rename = "swfPrecision")]
    pub swf_precision: f32,

    #[serde(rename = "textureGroupId")]
    pub texture_group_id: ResourceId,

    pub r#type: i32,

    #[serde(rename = "VTile")]
    pub v_tile: bool,

    pub width: i32,
}

impl Default for GMSprite {
    fn default() -> Self {
        let frame = GMSpriteFrame::default();

        Self {
            base: ResourceBase::new("Sprite1", "GMSprite"),
            resource_tag: "v2".into(),
            parent: ResourceId::default(),
            bbox_mode: 0,
            bbox_bottom: 0,
            bbox_left: 0,
            bbox_right: 0,
            bbox_top: 0,
            collision_kind: 1,
            collision_tolerance: 0,
            dynamic_texture_page: false,
            edge_filtering: false,
            for_3d: false,
            frames: vec![frame.clone()],
            grid_x: 0,
            grid_y: 0,
            height: 64,
            h_tile: false,
            layers: vec![GMImageLayer::default()],
            nine_slice: Value::Null,
            origin: 0,
            pre_multiply_alpha: false,
            sequence: GMSequence::new(
                vec![frame.base.name],
                &std::path::Path::new("sprites/Sprite1/Sprite1.yy"),
            ),
            swatch_colours: Value::Null,
            swf_precision: 0.5,
            texture_group_id: ResourceId {
                name: "Default".into(),
                path: "texturegroups/Default".into(),
            },
            r#type: 0,
            v_tile: false,
            width: 64,
        }
    }
}

impl ResourceTrait for GMSprite {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize Sprite");
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;

        for sprite_frame in &self.frames {
            sprite_frame
                .ensure_image_exists(path)
                .expect("Failed to ensure image exists for sprite frame");

            // All images must have same number of layers
            let sprite_frame_path = path.with_file_name(format!("{}.yy", &sprite_frame.base.name));
            for image_layer in &self.layers {
                image_layer
                    .ensure_image_exists(&sprite_frame_path)
                    .expect("Failed to ensure image exists for image layer");
            }
        }

        Ok(())
    }

    fn default_path(&self) -> String {
        format!("sprites/{}/{}.yy", self.name(), self.name())
    }
}

impl GMSprite {
    pub fn new(name: &str, parent: ResourceId, path: &std::path::Path) -> Self {
        let frame = GMSpriteFrame::default();

        Self {
            base: ResourceBase::new(name, "GMSprite"),
            frames: vec![frame.clone()],
            sequence: GMSequence::new(vec![frame.base.name], &path),
            parent,
            ..Default::default()
        }
    }

    pub fn load(path: Value) -> std::io::Result<Self> {
        let sprite: GMSprite = serde_json::from_value(path).expect("Failed to deserialize Sprite");
        Ok(sprite)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMSpriteFrame {
    #[serde(rename = "$GMSpriteFrame")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub base: ResourceBase,
}

impl Default for GMSpriteFrame {
    fn default() -> Self {
        let uuid = Uuid::new_v4().to_string();
        Self {
            resource_tag: "v1".into(),
            base: ResourceBase::new(uuid.as_str(), "GMSpriteFrame"),
        }
    }
}

impl GMSpriteFrame {
    pub fn new(name: &str) -> Self {
        Self {
            base: ResourceBase::new(name, "GMSpriteFrame"),
            ..Default::default()
        }
    }

    pub fn ensure_image_exists(
        &self,
        sprite_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // create blank 64x64 png image and save it to the sprite frame's path
        let img_path = sprite_path.with_file_name(format!("{}.png", self.base.name));
        if !img_path.exists() {
            let img = image::ImageBuffer::from_pixel(64, 64, image::Rgba([0u8, 0, 0, 0]));
            img.save(&img_path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMImageLayer {
    #[serde(rename = "$GMImageLayer")]
    pub resource_tag: String,
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

impl Default for GMImageLayer {
    fn default() -> Self {
        let uuid = Uuid::new_v4().to_string();
        Self {
            resource_tag: String::new(),
            base: ResourceBase::new(uuid.as_str(), "GMImageLayer"),
            blend_mode: 0,
            display_name: "default".into(),
            is_locked: false,
            opacity: 100.0,
            visible: true,
        }
    }
}

impl GMImageLayer {
    pub fn new() -> Self {
        GMImageLayer::default()
    }

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
        let img_path = dir_path.join(format!("{}.png", self.base.name));
        if !img_path.exists() {
            let img = image::ImageBuffer::from_pixel(64, 64, image::Rgba([0u8, 0, 0, 0]));
            img.save(&img_path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMSequence {
    #[serde(rename = "$GMSequence")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub base: ResourceBase,

    #[serde(rename = "autoRecord")]
    pub auto_record: bool,

    #[serde(rename = "backdropHeight")]
    pub backdrop_height: i32,
    #[serde(rename = "backdropImageOpacity")]
    pub backdrop_image_opacity: f32,
    #[serde(rename = "backdropImagePath")]
    pub backdrop_image_path: String,
    #[serde(rename = "backdropWidth")]
    pub backdrop_width: i32,
    #[serde(rename = "backdropXOffset")]
    pub backdrop_x_offset: f32,
    #[serde(rename = "backdropYOffset")]
    pub backdrop_y_offset: f32,

    pub events: MessageEventStore,

    #[serde(rename = "eventStubScript")]
    pub event_stub_script: Value,

    #[serde(rename = "eventToFunction")]
    pub event_to_function: HashMap<String, Value>,

    pub length: f32,

    #[serde(rename = "lockOrigin")]
    pub lock_origin: bool,

    pub moments: MomentsEventStore,

    pub playback: i32,
    #[serde(rename = "playbackSpeed")]
    pub playback_speed: f32,
    #[serde(rename = "playbackSpeedType")]
    pub playback_speed_type: i32,

    #[serde(rename = "showBackdrop")]
    pub show_backdrop: bool,
    #[serde(rename = "showBackdropImage")]
    pub show_backdrop_image: bool,

    #[serde(rename = "timeUnits")]
    pub time_units: i32,

    pub tracks: Vec<GMSpriteFramesTrack>,

    #[serde(rename = "visibleRange")]
    pub visible_range: Value,

    pub volume: f32,

    #[serde(rename = "xorigin")]
    pub x_origin: i32,
    #[serde(rename = "yorigin")]
    pub y_origin: i32,
}

impl Default for GMSequence {
    fn default() -> Self {
        Self {
            resource_tag: "v1".into(),
            base: ResourceBase::new("Sprite1", "GMSequence"),
            auto_record: true,
            backdrop_height: 768,
            backdrop_image_opacity: 0.5,
            backdrop_image_path: "".into(),
            backdrop_width: 1336,
            backdrop_x_offset: 0.0,
            backdrop_y_offset: 0.0,
            events: MessageEventStore::default(),
            event_stub_script: Value::Null,
            event_to_function: HashMap::new(),
            length: 1.0,
            lock_origin: false,
            moments: MomentsEventStore::default(),
            playback: 1,
            playback_speed: 30.0,
            playback_speed_type: 0,
            show_backdrop: true,
            show_backdrop_image: false,
            time_units: 1,
            tracks: vec![GMSpriteFramesTrack::default()],
            visible_range: Value::Null,
            volume: 1.0,
            x_origin: 0,
            y_origin: 0,
        }
    }
}

impl GMSequence {
    pub fn new(frames: Vec<String>, sprite_path: &std::path::Path) -> Self {
        Self {
            tracks: vec![GMSpriteFramesTrack::new(SpriteFrameStore::new(
                frames
                    .into_iter()
                    .map(|frame_name| {
                        let mut channels = HashMap::new();
                        channels.insert(
                            "0".to_string(),
                            SpriteFrameChannel::new(
                                frame_name.clone(),
                                sprite_path.to_string_lossy().to_string(),
                            ),
                        );
                        SpriteFrameKeyframe::new(channels)
                    })
                    .collect(),
            ))],
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageEventStore {
    #[serde(rename = "$KeyframeStore<MessageEventKeyframe>")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<Value>,
}

impl Default for MessageEventStore {
    fn default() -> Self {
        Self {
            resource_tag: String::new(),
            resource_type: ResourceType::new("KeyframeStore<MessageEventKeyframe>"),
            keyframes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MomentsEventStore {
    #[serde(rename = "$KeyframeStore<MomentsEventKeyframe>")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<Value>,
}

impl Default for MomentsEventStore {
    fn default() -> Self {
        Self {
            resource_tag: String::new(),
            resource_type: ResourceType::new("KeyframeStore<MomentsEventKeyframe>"),
            keyframes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMSpriteFramesTrack {
    #[serde(rename = "$GMSpriteFramesTrack")]
    pub resource_tag: String,
    #[serde(rename = "builtinName")]
    pub builtin_name: i32,
    pub name: String,
    #[serde(flatten)]
    pub resource_type: ResourceType,

    pub events: Vec<Value>,

    #[serde(rename = "inheritsTrackColour")]
    pub inherits_track_colour: bool,
    pub interpolation: i32,
    #[serde(rename = "isCreationTrack")]
    pub is_creation_track: bool,

    pub keyframes: SpriteFrameStore,

    pub modifiers: Vec<Value>,

    #[serde(rename = "spriteId")]
    pub sprite_id: Value,

    #[serde(rename = "trackColour")]
    pub track_colour: i32,

    pub tracks: Vec<Value>,

    pub traits: i32,
}

impl Default for GMSpriteFramesTrack {
    fn default() -> Self {
        Self {
            resource_tag: String::new(),
            builtin_name: 0,
            name: "frames".into(),
            resource_type: ResourceType::new("GMSpriteFramesTrack"),

            events: Vec::new(),
            inherits_track_colour: true,
            interpolation: 1,
            is_creation_track: false,
            keyframes: SpriteFrameStore::default(),
            modifiers: Vec::new(),
            sprite_id: Value::Null,
            track_colour: 0,
            tracks: Vec::new(),
            traits: 0,
        }
    }
}

impl GMSpriteFramesTrack {
    pub fn new(keyframes: SpriteFrameStore) -> Self {
        Self {
            keyframes,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteFrameStore {
    #[serde(rename = "$KeyframeStore<SpriteFrameKeyframe>")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<SpriteFrameKeyframe>,
}

impl Default for SpriteFrameStore {
    fn default() -> Self {
        Self {
            resource_tag: String::new(),
            resource_type: ResourceType::new("KeyframeStore<SpriteFrameKeyframe>"),
            keyframes: vec![SpriteFrameKeyframe::default()],
        }
    }
}

impl SpriteFrameStore {
    pub fn new(keyframes: Vec<SpriteFrameKeyframe>) -> Self {
        Self {
            keyframes,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteFrameKeyframe {
    #[serde(rename = "$Keyframe<SpriteFrameKeyframe>")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "Channels")]
    pub channels: HashMap<String, SpriteFrameChannel>,

    #[serde(rename = "Disabled")]
    pub disabled: bool,

    pub id: String,

    #[serde(rename = "IsCreationKey")]
    pub is_creation_key: bool,

    #[serde(rename = "Key")]
    pub key: f32,

    #[serde(rename = "Length")]
    pub length: f32,

    #[serde(rename = "Stretch")]
    pub stretch: bool,
}

impl Default for SpriteFrameKeyframe {
    fn default() -> Self {
        Self {
            resource_tag: String::new(),
            resource_type: ResourceType::new("Keyframe<SpriteFrameKeyframe>"),
            channels: [("0".to_string(), SpriteFrameChannel::default())]
                .iter()
                .cloned()
                .collect(),
            disabled: false,
            id: Uuid::new_v4().into(),
            is_creation_key: false,
            key: 0.0,
            length: 1.0,
            stretch: false,
        }
    }
}

impl SpriteFrameKeyframe {
    pub fn new(channels: HashMap<String, SpriteFrameChannel>) -> Self {
        Self {
            channels,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteFrameChannel {
    #[serde(rename = "$SpriteFrameKeyframe")]
    pub resource_tag: String,
    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "Id")]
    pub id: ResourceId,
}

impl Default for SpriteFrameChannel {
    fn default() -> Self {
        let uuid = Uuid::new_v4().to_string();
        let sprite_default_path = "sprites/Sprite1/Sprite1.yy";
        let resource_id = ResourceId {
            name: uuid,
            path: sprite_default_path.into(),
        };

        Self {
            resource_tag: String::new(),
            resource_type: ResourceType::new("SpriteFrameKeyframe"),
            id: resource_id,
        }
    }
}

impl SpriteFrameChannel {
    pub fn new(name: String, path: String) -> Self {
        let id = ResourceId {
            name: name,
            path: path,
        };
        Self {
            id,
            ..Default::default()
        }
    }
}
