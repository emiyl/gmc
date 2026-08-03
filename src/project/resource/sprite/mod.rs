use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::project::ResourceId;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GmSprite {
    #[serde(rename = "$GMSprite")]
    pub gm_sprite: String,

    #[serde(rename = "%Name")]
    pub display_name: String,

    #[serde(default, rename = "bboxMode")]
    pub bbox_mode: i32,
    #[serde(default)]
    pub bbox_bottom: i32,
    #[serde(default)]
    pub bbox_left: i32,
    #[serde(default)]
    pub bbox_right: i32,
    #[serde(default)]
    pub bbox_top: i32,

    #[serde(default, rename = "collisionKind")]
    pub collision_kind: i32,
    #[serde(default, rename = "collisionTolerance")]
    pub collision_tolerance: i32,

    #[serde(default, rename = "DynamicTexturePage")]
    pub dynamic_texture_page: bool,
    #[serde(default, rename = "edgeFiltering")]
    pub edge_filtering: bool,
    #[serde(default, rename = "For3D")]
    pub for_3d: bool,

    #[serde(default)]
    pub frames: Vec<GmSpriteFrame>,

    #[serde(default, rename = "gridX")]
    pub grid_x: i32,
    #[serde(default, rename = "gridY")]
    pub grid_y: i32,

    #[serde(default)]
    pub height: i32,

    #[serde(default, rename = "HTile")]
    pub h_tile: bool,

    #[serde(default)]
    pub layers: Vec<GmImageLayer>,

    pub name: String,

    #[serde(default, rename = "nineSlice")]
    pub nine_slice: Option<serde_json::Value>,

    #[serde(default)]
    pub origin: i32,

    pub parent: ResourceId,

    #[serde(default, rename = "preMultiplyAlpha")]
    pub pre_multiply_alpha: bool,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    pub sequence: GmSequence,

    #[serde(rename = "swatchColours")]
    pub swatch_colours: Option<serde_json::Value>,

    #[serde(default, rename = "swfPrecision")]
    pub swf_precision: f32,

    #[serde(rename = "textureGroupId")]
    pub texture_group_id: ResourceId,

    #[serde(default)]
    pub r#type: i32,

    #[serde(default, rename = "VTile")]
    pub v_tile: bool,

    #[serde(default)]
    pub width: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GmSpriteFrame {
    #[serde(rename = "$GMSpriteFrame")]
    pub gm_sprite_frame: String,

    #[serde(rename = "%Name")]
    pub display_name: String,

    pub name: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GmImageLayer {
    #[serde(rename = "$GMImageLayer")]
    pub gm_image_layer: String,

    #[serde(rename = "%Name")]
    pub display_name: String,

    #[serde(rename = "blendMode")]
    pub blend_mode: i32,
    #[serde(rename = "displayName")]
    pub display_name_override: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GmSequence {
    #[serde(rename = "$GMSequence")]
    pub gm_sequence: String,

    #[serde(rename = "%Name")]
    pub display_name: String,

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
    pub event_stub_script: Option<serde_json::Value>,

    #[serde(default, rename = "eventToFunction")]
    pub event_to_function: HashMap<String, serde_json::Value>,

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
    pub visible_range: Option<serde_json::Value>,

    pub volume: f32,

    #[serde(rename = "xorigin")]
    pub x_origin: i32,
    #[serde(rename = "yorigin")]
    pub y_origin: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpriteFramesTrack {
    #[serde(rename = "$GMSpriteFramesTrack")]
    pub gm_track: String,

    #[serde(rename = "builtinName")]
    pub builtin_name: i32,

    #[serde(default)]
    pub events: Vec<serde_json::Value>,

    #[serde(rename = "inheritsTrackColour")]
    pub inherits_track_colour: bool,
    pub interpolation: i32,
    #[serde(rename = "isCreationTrack")]
    pub is_creation_track: bool,

    pub keyframes: SpriteFrameStore,

    #[serde(default)]
    pub modifiers: Vec<serde_json::Value>,

    pub name: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    #[serde(rename = "spriteId")]
    pub sprite_id: Option<serde_json::Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: i32,

    #[serde(default)]
    pub tracks: Vec<serde_json::Value>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
