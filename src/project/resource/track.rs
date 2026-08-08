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

impl<T> Default for KeyframeStore<T> {
    fn default() -> Self {
        Self {
            resource_tag: HashMap::new(),
            resource_type: ResourceType::default(),
            keyframes: Vec::new(),
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
    GMAudioEffectTrack(GMAudioEffectTrack),
    GMBoolTrack(GMBoolTrack),
    GMColorTrack(GMColorTrack),
    GMStringTrack(GMStringTrack),
    GMAudioTrack(GMAudioTrack),
    GMGraphicTrack(GMGraphicTrack),
    GMParticleTrack(GMParticleTrack),
    GMSequenceTrack(GMSequenceTrack),
    GMTextTrack(GMTextTrack),
    GMComponentTrack(GMComponentTrack),
    GMParameterTrack(GMParameterTrack),
    GMResourceTrack(GMResourceTrack),
    GMGroupTrack(GMGroupTrack),
    GMClippingMaskTrack(GMClippingMaskTrack),
    GMClippingMaskMask(GMClippingMaskMask),
    GMClippingMaskSubject(GMClippingMaskSubject),
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
pub struct AudioKeyframe {
    #[serde(rename = "SoundId", default)]
    pub sound_id: Option<ResourceId>,

    #[serde(rename = "EmitterId", default)]
    pub emitter_id: Option<ResourceId>,

    #[serde(rename = "Mode", default)]
    pub mode: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioEffectKeyframe {
    #[serde(rename = "Parameter", default)]
    pub parameter: Option<Value>,

    #[serde(rename = "AnimCurveId", default)]
    pub anim_curve_id: Option<Value>,

    #[serde(rename = "EmbeddedAnimCurve", default)]
    pub embedded_anim_curve: Option<Value>,

    #[serde(rename = "IsCurveEmbedded", default)]
    pub is_curve_embedded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoolKeyframe {
    #[serde(rename = "Value", default)]
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorKeyframe {
    #[serde(rename = "Color", default)]
    pub color: u32,

    #[serde(rename = "AnimCurveId", default)]
    pub anim_curve_id: Option<Value>,

    #[serde(rename = "EmbeddedAnimCurve", default)]
    pub embedded_anim_curve: Option<Value>,

    #[serde(rename = "IsCurveEmbedded", default)]
    pub is_curve_embedded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StringKeyframe {
    #[serde(rename = "String", default)]
    pub string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextKeyframe {
    #[serde(rename = "Text", default)]
    pub text: String,

    #[serde(rename = "Wrap", default)]
    pub wrap: bool,

    #[serde(rename = "WrapMode", default)]
    pub wrap_mode: i32,

    #[serde(rename = "Alignment", default)]
    pub alignment: i32,

    #[serde(rename = "Origin", default)]
    pub origin: i32,

    #[serde(rename = "Id", default)]
    pub id: ResourceId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetKeyframe {
    #[serde(rename = "Id")]
    pub id: ResourceId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMAudioEffectTrack {
    #[serde(rename = "$GMAudioEffectTrack")]
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

    pub keyframes: KeyframeStore<AudioEffectKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMBoolTrack {
    #[serde(rename = "$GMBoolTrack")]
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

    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<BoolKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMColorTrack {
    #[serde(rename = "$GMColorTrack")]
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

    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<ColorKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMStringTrack {
    #[serde(rename = "$GMStringTrack")]
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

    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<StringKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMAudioTrack {
    #[serde(rename = "$GMAudioTrack")]
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

    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<AudioKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMGraphicTrack {
    #[serde(rename = "$GMGraphicTrack")]
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

    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<AssetKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMParticleTrack {
    #[serde(rename = "$GMParticleTrack")]
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

    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<AssetKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMSequenceTrack {
    #[serde(rename = "$GMSequenceTrack")]
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

    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<AssetKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMTextTrack {
    #[serde(rename = "$GMTextTrack")]
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

    pub is_creation_track: bool,

    pub keyframes: KeyframeStore<TextKeyframe>,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMComponentTrack {
    #[serde(rename = "$GMComponentTrack")]
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

    pub is_creation_track: bool,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMParameterTrack {
    #[serde(rename = "$GMParameterTrack")]
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

    pub is_creation_track: bool,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMResourceTrack {
    #[serde(rename = "$GMResourceTrack")]
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

    pub is_creation_track: bool,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMGroupTrack {
    #[serde(rename = "$GMGroupTrack")]
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

    pub is_creation_track: bool,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMClippingMaskTrack {
    #[serde(rename = "$GMClippingMaskTrack")]
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

    pub is_creation_track: bool,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMClippingMaskMask {
    #[serde(rename = "$GMClippingMask_Mask")]
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

    pub is_creation_track: bool,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GMClippingMaskSubject {
    #[serde(rename = "$GMClippingMask_Subject")]
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

    pub is_creation_track: bool,

    pub modifiers: Vec<Value>,

    #[serde(rename = "trackColour")]
    pub track_colour: u32,

    pub tracks: Vec<TrackResource>,

    pub traits: i32,
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
