use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::project::ResourceId;
use crate::project::resource::ResourceType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyframeStore<T> {
    #[serde(flatten)]
    pub resource_tag: HashMap<String, Value>,

    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<Keyframe<T>>,
}

impl<T> KeyframeStore<T> {
    pub fn new(resource_type: &str, keyframes: Vec<Keyframe<T>>) -> Self {
        let mut resource_tag = HashMap::new();
        resource_tag.insert(format!("${}", resource_type), Value::String(String::new()));

        Self {
            resource_tag,
            resource_type: ResourceType::new(resource_type),
            keyframes,
        }
    }
}

pub type SpriteFrameStore = KeyframeStore<SpriteFrameChannel>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Keyframe<T> {
    #[serde(flatten)]
    pub resource_tag: HashMap<String, Value>,

    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "Channels")]
    pub channels: HashMap<String, T>,

    #[serde(rename = "Disabled")]
    pub disabled: bool,

    pub id: String,

    #[serde(rename = "IsCreationKey")]
    pub is_creation_key: bool,

    #[serde(rename = "Key")]
    pub key: f64,

    #[serde(rename = "Length")]
    pub length: f64,

    #[serde(rename = "Stretch")]
    pub stretch: bool,
}

impl<T> Keyframe<T> {
    pub fn new(resource_type: &str, channels: HashMap<String, T>, key: f64) -> Self {
        let mut resource_tag = HashMap::new();
        resource_tag.insert(format!("${}", resource_type), Value::String(String::new()));

        Self {
            resource_tag,
            resource_type: ResourceType::new(resource_type),
            channels,
            disabled: false,
            id: String::new(),
            is_creation_key: false,
            key,
            length: 1.0,
            stretch: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TrackResource {
    GMInstanceTrack(GMInstanceTrack),
    GMRealTrack(GMRealTrack),
    GMSpriteFramesTrack(GMSpriteFramesTrack),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMInstanceTrack {
    #[serde(rename = "$GMInstanceTrack")]
    pub resource_tag: String,

    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "%Name")]
    pub display_name: String,

    #[serde(rename = "builtinName")]
    pub builtin_name: i32,

    pub name: String,

    pub events: Vec<Value>,

    #[serde(rename = "inheritsTrackColour")]
    pub inherits_track_colour: bool,

    pub interpolation: i32,

    #[serde(rename = "isCreationTrack")]
    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<AssetInstanceKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetInstanceKeyframe {
    #[serde(rename = "Id")]
    pub id: ResourceId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMRealTrack {
    #[serde(rename = "$GMRealTrack")]
    pub resource_tag: String,

    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "%Name")]
    pub display_name: String,

    #[serde(rename = "builtinName")]
    pub builtin_name: i32,

    pub name: String,

    pub events: Vec<Value>,

    #[serde(rename = "inheritsTrackColour")]
    pub inherits_track_colour: bool,

    pub interpolation: i32,

    #[serde(rename = "isCreationTrack")]
    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<RealKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RealKeyframe {
    #[serde(rename = "AnimCurveId")]
    pub anim_curve_id: Option<Value>,

    #[serde(rename = "EmbeddedAnimCurve")]
    pub embedded_anim_curve: Option<Value>,

    #[serde(rename = "RealValue")]
    pub real_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMSpriteFramesTrack {
    #[serde(rename = "$GMSpriteFramesTrack")]
    pub resource_tag: String,

    #[serde(flatten)]
    pub resource_type: ResourceType,

    #[serde(rename = "builtinName")]
    pub builtin_name: i32,

    pub name: String,

    pub events: Vec<Value>,

    #[serde(rename = "inheritsTrackColour")]
    pub inherits_track_colour: bool,

    pub interpolation: i32,

    #[serde(rename = "isCreationTrack")]
    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<SpriteFrameChannel>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "spriteId")]
    pub sprite_id: Option<ResourceId>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

impl GMSpriteFramesTrack {
    pub fn new(keyframes: SpriteFrameStore) -> Self {
        Self {
            resource_tag: String::new(),
            resource_type: ResourceType::new("GMSpriteFramesTrack"),
            builtin_name: 0,
            name: "frames".to_string(),
            events: Vec::new(),
            inherits_track_colour: true,
            interpolation: 1,
            is_creation_track: false,
            keyframes,
            modifiers: Vec::new(),
            sprite_id: None,
            track_colour: 0,
            tracks: Vec::new(),
            traits: 0,
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
        Self {
            resource_tag: String::new(),
            resource_type: ResourceType::new("SpriteFrameKeyframe"),
            id: ResourceId::default(),
        }
    }
}

impl SpriteFrameChannel {
    pub fn new(name: String, path: String) -> Self {
        Self {
            id: ResourceId { name, path },
            ..Default::default()
        }
    }
}
