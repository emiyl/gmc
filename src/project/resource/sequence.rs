use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::project::resource::{
    ResourceBase, ResourceType,
    track::{GMSpriteFramesTrack, Keyframe, SpriteFrameChannel, SpriteFrameStore},
};

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
            tracks: Vec::new(),
            visible_range: Value::Null,
            volume: 1.0,
            x_origin: 0,
            y_origin: 0,
        }
    }
}

impl GMSequence {
    pub fn new(frames: Vec<String>, sprite_path: &std::path::Path) -> Self {
        let keyframes: Vec<Keyframe<SpriteFrameChannel>> = frames
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, frame_name)| {
                let mut channels = HashMap::new();
                channels.insert(
                    "0".to_string(),
                    SpriteFrameChannel::new(
                        frame_name.clone(),
                        sprite_path.to_string_lossy().to_string(),
                    ),
                );
                Keyframe::new("Keyframe<SpriteFrameKeyframe>", channels, i as f64)
            })
            .collect();

        Self {
            tracks: vec![GMSpriteFramesTrack::new(SpriteFrameStore::new(
                "KeyframeStore<SpriteFrameKeyframe>",
                keyframes,
            ))],
            length: frames.len() as f32,
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
