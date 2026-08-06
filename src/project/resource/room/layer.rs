use crate::project::resource::ResourceBase;

use super::instance::GMRInstance;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GMRInstanceLayer {
    // resource_tag is taken care of by serialize and deserialize implementations for Layer
    #[serde(flatten)]
    pub base: ResourceBase,

    pub depth: i32,

    #[serde(rename = "effectEnabled")]
    pub effect_enabled: bool,

    #[serde(rename = "effectType")]
    pub effect_type: Value,

    #[serde(rename = "gridX")]
    pub grid_x: i32,

    #[serde(rename = "gridY")]
    pub grid_y: i32,

    #[serde(rename = "hierarchyFrozen")]
    pub hierarchy_frozen: bool,

    #[serde(rename = "inheritLayerDepth")]
    pub inherit_layer_depth: bool,
    #[serde(rename = "inheritLayerSettings")]
    pub inherit_layer_settings: bool,
    #[serde(rename = "inheritSubLayers")]
    pub inherit_sub_layers: bool,
    #[serde(rename = "inheritVisibility")]
    pub inherit_visibility: bool,

    pub instances: Vec<GMRInstance>,
    pub layers: Vec<Layer>,

    pub properties: Vec<Value>,

    #[serde(rename = "userdefinedDepth")]
    pub user_defined_depth: bool,
    pub visible: bool,
}

impl Default for GMRInstanceLayer {
    fn default() -> Self {
        Self {
            base: ResourceBase::new("Instances", "GMRInstanceLayer"),

            depth: 0,
            effect_enabled: true,
            effect_type: Value::Null,

            grid_x: 32,
            grid_y: 32,
            hierarchy_frozen: false,

            inherit_layer_depth: false,
            inherit_layer_settings: false,
            inherit_sub_layers: true,
            inherit_visibility: true,

            instances: Vec::new(),
            layers: Vec::new(),

            properties: Vec::new(),

            user_defined_depth: false,
            visible: true,
        }
    }
}

impl GMRInstanceLayer {
    pub fn new(depth: i32) -> Self {
        Self {
            depth,
            ..Default::default()
        }
    }

    pub fn add_instance(&mut self, instance: GMRInstance) {
        self.instances.push(instance);
    }

    pub fn with_instance(mut self, instance: GMRInstance) -> Self {
        self.add_instance(instance);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GMRBackgroundLayer {
    #[serde(flatten)]
    pub base: ResourceBase,

    #[serde(rename = "animationFPS")]
    pub animation_fps: f64,

    #[serde(rename = "animationSpeedType")]
    pub animation_speed_type: i32,

    pub colour: u32,
    pub depth: i32,

    #[serde(rename = "effectEnabled")]
    pub effect_enabled: bool,
    #[serde(rename = "effectType")]
    pub effect_type: Value,

    #[serde(rename = "gridX")]
    pub grid_x: i32,
    #[serde(rename = "gridY")]
    pub grid_y: i32,

    #[serde(rename = "hierarchyFrozen")]
    pub hierarchy_frozen: bool,

    #[serde(rename = "hspeed")]
    pub hspeed: f64,

    #[serde(rename = "htiled")]
    pub htiled: bool,

    #[serde(rename = "inheritLayerDepth")]
    pub inherit_layer_depth: bool,
    #[serde(rename = "inheritLayerSettings")]
    pub inherit_layer_settings: bool,
    #[serde(rename = "inheritSubLayers")]
    pub inherit_sub_layers: bool,
    #[serde(rename = "inheritVisibility")]
    pub inherit_visibility: bool,

    pub layers: Vec<Layer>,
    pub properties: Vec<Value>,

    #[serde(rename = "spriteId")]
    pub sprite_id: Value,
    #[serde(rename = "stretch")]
    pub stretch: bool,

    #[serde(rename = "userdefinedAnimFPS")]
    pub user_defined_anim_fps: bool,
    #[serde(rename = "userdefinedDepth")]
    pub user_defined_depth: bool,

    pub visible: bool,
    pub vspeed: f64,
    pub vtiled: bool,

    pub x: i32,
    pub y: i32,
}

impl Default for GMRBackgroundLayer {
    fn default() -> Self {
        Self {
            base: ResourceBase::new("Background", "GMRBackgroundLayer"),

            animation_fps: 15.0,
            animation_speed_type: 0,

            colour: 0xFF000000,
            depth: 100,

            effect_enabled: true,
            effect_type: Value::Null,

            grid_x: 32,
            grid_y: 32,

            hierarchy_frozen: false,
            hspeed: 0.0,
            htiled: false,

            inherit_layer_depth: false,
            inherit_layer_settings: false,
            inherit_sub_layers: true,
            inherit_visibility: true,

            layers: Vec::new(),
            properties: Vec::new(),

            sprite_id: Value::Null,
            stretch: false,

            user_defined_anim_fps: false,
            user_defined_depth: false,

            visible: true,
            vspeed: 0.0,
            vtiled: false,

            x: 0,
            y: 0,
        }
    }
}

impl GMRBackgroundLayer {
    pub fn new(depth: i32) -> Self {
        Self {
            depth,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Layer {
    Instance(GMRInstanceLayer),
    Background(GMRBackgroundLayer),
}

pub trait LayerTrait {
    fn name(&self) -> &str;
    fn instances(&self) -> Option<&Vec<GMRInstance>>;
}

impl LayerTrait for Layer {
    fn name(&self) -> &str {
        match self {
            Layer::Instance(layer) => &layer.base.name,
            Layer::Background(layer) => &layer.base.name,
        }
    }

    fn instances(&self) -> Option<&Vec<GMRInstance>> {
        match self {
            Layer::Instance(layer) => Some(&layer.instances),
            Layer::Background(_) => None,
        }
    }
}

impl Default for Layer {
    fn default() -> Self {
        Layer::Instance(GMRInstanceLayer::default())
    }
}

impl Layer {
    pub fn instance_layer(depth: i32) -> Self {
        Self::Instance(GMRInstanceLayer::new(depth))
    }

    pub fn background_layer(depth: i32) -> Self {
        Self::Background(GMRBackgroundLayer::new(depth))
    }
}

impl Serialize for Layer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Layer::Instance(layer) => {
                let mut value = serde_json::to_value(layer).map_err(serde::ser::Error::custom)?;

                if let Value::Object(ref mut map) = value {
                    map.insert(
                        "$GMRInstanceLayer".to_string(),
                        Value::String(String::new()),
                    );

                    map.insert(
                        "resourceType".to_string(),
                        Value::String("GMRInstanceLayer".to_string()),
                    );

                    map.insert(
                        "resourceVersion".to_string(),
                        Value::String("2.0".to_string()),
                    );
                }

                value.serialize(serializer)
            }

            Layer::Background(layer) => {
                let mut value = serde_json::to_value(layer).map_err(serde::ser::Error::custom)?;

                if let Value::Object(ref mut map) = value {
                    map.insert(
                        "$GMRBackgroundLayer".to_string(),
                        Value::String(String::new()),
                    );

                    map.insert(
                        "resourceType".to_string(),
                        Value::String("GMRBackgroundLayer".to_string()),
                    );

                    map.insert(
                        "resourceVersion".to_string(),
                        Value::String("2.0".to_string()),
                    );
                }

                value.serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Layer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        match value.get("$GMRInstanceLayer") {
            Some(_) => {
                let layer = serde_json::from_value::<GMRInstanceLayer>(value)
                    .map_err(serde::de::Error::custom)?;

                Ok(Layer::Instance(layer))
            }

            None => match value.get("$GMRBackgroundLayer") {
                Some(_) => {
                    let layer = serde_json::from_value::<GMRBackgroundLayer>(value)
                        .map_err(serde::de::Error::custom)?;

                    Ok(Layer::Background(layer))
                }

                None => Err(serde::de::Error::custom("unknown GameMaker layer type")),
            },
        }
    }
}
