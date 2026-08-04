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

use crate::project::{ResourceId, ResourceTrait, formatter::format_gamemaker_json};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sprite {
    #[serde(rename = "$GMSprite")]
    pub gm_sprite: String,

    #[serde(rename = "%Name")]
    pub percent_name: String,

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

    pub frames: Vec<SpriteFrame>,

    #[serde(rename = "gridX")]
    pub grid_x: i32,
    #[serde(rename = "gridY")]
    pub grid_y: i32,

    pub height: i32,

    #[serde(rename = "HTile")]
    pub h_tile: bool,

    pub layers: Vec<ImageLayer>,

    pub name: String,

    #[serde(rename = "nineSlice")]
    pub nine_slice: Value,

    pub origin: i32,

    pub parent: ResourceId,

    #[serde(rename = "preMultiplyAlpha")]
    pub pre_multiply_alpha: bool,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    pub sequence: Sequence,

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

impl Default for Sprite {
    fn default() -> Self {
        Self {
            gm_sprite: "v2".into(),
            percent_name: "Sprite1".into(),
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
            frames: vec![SpriteFrame::default()],
            grid_x: 0,
            grid_y: 0,
            height: 64,
            h_tile: false,
            layers: vec![ImageLayer::default()],
            name: "Sprite1".into(),
            nine_slice: Value::Null,
            origin: 0,
            parent: ResourceId {
                name: "".into(),
                path: "".into(),
            },
            pre_multiply_alpha: false,
            resource_type: "GMSprite".into(),
            resource_version: "2.0".into(),
            sequence: Sequence::default(),
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

impl ResourceTrait for Sprite {
    fn name(&self) -> &str {
        &self.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize Sprite");
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;

        Ok(())
    }

    fn default_path(&self) -> String {
        format!("sprites/{}/{}.yy", self.name, self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteFrame {
    #[serde(rename = "$GMSpriteFrame")]
    pub gm_sprite_frame: String,

    #[serde(rename = "%Name")]
    pub percent_name: String,

    pub name: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for SpriteFrame {
    fn default() -> Self {
        let uuid = Uuid::new_v4().to_string();
        Self {
            gm_sprite_frame: "v1".into(),
            percent_name: uuid.clone(),
            name: uuid.clone(),
            resource_type: "GMSpriteFrame".into(),
            resource_version: "2.0".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageLayer {
    #[serde(rename = "$GMImageLayer")]
    pub gm_image_layer: String,

    #[serde(rename = "%Name")]
    pub percent_name: String,

    #[serde(rename = "blendMode")]
    pub blend_mode: i32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub name: String,
    pub opacity: f32,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    pub visible: bool,
}

impl Default for ImageLayer {
    fn default() -> Self {
        let uuid = Uuid::new_v4().to_string();
        Self {
            gm_image_layer: "".into(),
            percent_name: uuid.clone(),
            blend_mode: 0,
            display_name: "default".into(),
            is_locked: false,
            name: uuid.clone(),
            opacity: 100.0,
            resource_type: "GMImageLayer".into(),
            resource_version: "2.0".into(),
            visible: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sequence {
    #[serde(rename = "$GMSequence")]
    pub gm_sequence: String,

    #[serde(rename = "%Name")]
    pub percent_name: String,

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

    pub name: String,

    pub playback: i32,
    #[serde(rename = "playbackSpeed")]
    pub playback_speed: f32,
    #[serde(rename = "playbackSpeedType")]
    pub playback_speed_type: i32,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    #[serde(rename = "showBackdrop")]
    pub show_backdrop: bool,
    #[serde(rename = "showBackdropImage")]
    pub show_backdrop_image: bool,

    #[serde(rename = "timeUnits")]
    pub time_units: i32,

    pub tracks: Vec<SpriteFramesTrack>,

    #[serde(rename = "visibleRange")]
    pub visible_range: Value,

    pub volume: f32,

    #[serde(rename = "xorigin")]
    pub x_origin: i32,
    #[serde(rename = "yorigin")]
    pub y_origin: i32,
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            gm_sequence: "v1".into(),
            percent_name: "Sprite1".into(),
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
            name: "Sprite1".into(),
            playback: 1,
            playback_speed: 30.0,
            playback_speed_type: 0,
            resource_type: "GMSequence".into(),
            resource_version: "2.0".into(),
            show_backdrop: true,
            show_backdrop_image: false,
            time_units: 1,
            tracks: vec![SpriteFramesTrack::default()],
            visible_range: Value::Null,
            volume: 1.0,
            x_origin: 0,
            y_origin: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageEventStore {
    #[serde(rename = "$KeyframeStore<MessageEventKeyframe>")]
    pub keyframe_store: String,

    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<serde_json::Value>,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for MessageEventStore {
    fn default() -> Self {
        Self {
            keyframe_store: "".into(),
            keyframes: Vec::new(),
            resource_type: "KeyframeStore<MessageEventKeyframe>".into(),
            resource_version: "2.0".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MomentsEventStore {
    #[serde(rename = "$KeyframeStore<MomentsEventKeyframe>")]
    pub keyframe_store: String,

    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<serde_json::Value>,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for MomentsEventStore {
    fn default() -> Self {
        Self {
            keyframe_store: "".into(),
            keyframes: Vec::new(),
            resource_type: "KeyframeStore<MomentsEventKeyframe>".into(),
            resource_version: "2.0".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteFramesTrack {
    #[serde(rename = "$GMSpriteFramesTrack")]
    pub gm_track: String,

    #[serde(rename = "builtinName")]
    pub builtin_name: i32,

    pub events: Vec<serde_json::Value>,

    #[serde(rename = "inheritsTrackColour")]
    pub inherits_track_colour: bool,
    pub interpolation: i32,
    #[serde(rename = "isCreationTrack")]
    pub is_creation_track: bool,

    pub keyframes: SpriteFrameStore,

    pub modifiers: Vec<serde_json::Value>,

    pub name: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    #[serde(rename = "spriteId")]
    pub sprite_id: Value,

    #[serde(rename = "trackColour")]
    pub track_colour: i32,

    pub tracks: Vec<serde_json::Value>,

    pub traits: i32,
}

impl Default for SpriteFramesTrack {
    fn default() -> Self {
        Self {
            gm_track: "".into(),
            builtin_name: 0,
            events: Vec::new(),
            inherits_track_colour: true,
            interpolation: 1,
            is_creation_track: false,
            keyframes: SpriteFrameStore::default(),
            modifiers: Vec::new(),
            name: "frames".into(),
            resource_type: "GMSpriteFramesTrack".into(),
            resource_version: "2.0".into(),
            sprite_id: Value::Null,
            track_colour: 0,
            tracks: Vec::new(),
            traits: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteFrameStore {
    #[serde(rename = "$KeyframeStore<SpriteFrameKeyframe>")]
    pub keyframe_store: String,

    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<SpriteFrameKeyframe>,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for SpriteFrameStore {
    fn default() -> Self {
        Self {
            keyframe_store: "".into(),
            keyframes: vec![SpriteFrameKeyframe::default()],
            resource_type: "KeyframeStore<SpriteFrameKeyframe>".into(),
            resource_version: "2.0".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteFrameKeyframe {
    #[serde(rename = "$Keyframe<SpriteFrameKeyframe>")]
    pub gm_keyframe: String,

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

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    #[serde(rename = "Stretch")]
    pub stretch: bool,
}

impl Default for SpriteFrameKeyframe {
    fn default() -> Self {
        Self {
            gm_keyframe: "".into(),
            channels: [("0".to_string(), SpriteFrameChannel::default())]
                .iter()
                .cloned()
                .collect(),
            disabled: false,
            id: Uuid::new_v4().into(),
            is_creation_key: false,
            key: 0.0,
            length: 1.0,
            resource_type: "Keyframe<SpriteFrameKeyframe>".into(),
            resource_version: "2.0".into(),
            stretch: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteFrameChannel {
    #[serde(rename = "$SpriteFrameKeyframe")]
    pub sprite_frame_keyframe: String,

    #[serde(rename = "Id")]
    pub id: ResourceId,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for SpriteFrameChannel {
    fn default() -> Self {
        let uuid = Uuid::new_v4().to_string();
        let sprite_default_path = Sprite::default().default_path();
        let resource_id = ResourceId {
            name: uuid,
            path: sprite_default_path,
        };

        Self {
            sprite_frame_keyframe: "".into(),
            id: resource_id,
            resource_type: "SpriteFrameKeyframe".into(),
            resource_version: "2.0".into(),
        }
    }
}
