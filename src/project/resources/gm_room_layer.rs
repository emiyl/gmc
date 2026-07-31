use super::ResourceRef;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Instance {
    #[serde(rename = "$GMRInstance")]
    pub resource_tag: String,

    #[serde(rename = "%Name")]
    pub display_name: String,

    pub name: String,

    pub colour: u32,

    pub frozen: bool,

    #[serde(rename = "hasCreationCode")]
    pub has_creation_code: bool,

    pub ignore: bool,

    #[serde(rename = "imageIndex")]
    pub image_index: i32,

    #[serde(rename = "imageSpeed")]
    pub image_speed: f32,

    #[serde(rename = "inheritCode")]
    pub inherit_code: bool,

    #[serde(rename = "inheritedItemId")]
    pub inherited_item_id: Option<String>,

    #[serde(rename = "inheritItemSettings")]
    pub inherit_item_settings: bool,

    #[serde(rename = "isDnd")]
    pub is_dnd: bool,

    #[serde(rename = "objectId")]
    pub object: Option<ResourceRef>,

    pub properties: Vec<Value>,

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    pub rotation: f32,

    #[serde(rename = "scaleX")]
    pub scale_x: f32,

    #[serde(rename = "scaleY")]
    pub scale_y: f32,

    pub x: f32,
    pub y: f32,
}

impl Default for Instance {
    fn default() -> Self {
        let name = Self::new_instance_name();
        Self {
            resource_tag: "v4".to_string(),

            display_name: name.clone(),
            name: name.clone(),

            colour: 0xFFFFFFFF,
            frozen: false,

            has_creation_code: false,
            ignore: false,

            image_index: 0,
            image_speed: 1.0,

            inherit_code: false,
            inherited_item_id: None,
            inherit_item_settings: false,
            is_dnd: false,

            object: None,

            properties: Vec::new(),

            resource_type: "GMRInstance".into(),
            resource_version: "2.0".into(),

            rotation: 0.0,

            scale_x: 1.0,
            scale_y: 1.0,

            x: 0.0,
            y: 0.0,
        }
    }
}

impl Instance {
    fn new_instance_name() -> String {
        // Generate a random 8-digit hexadecimal string to use as a unique instance name.
        let value: u32 = rand::rng().random();
        format!("inst_{:08X}", value)
    }

    pub fn new(object: ResourceRef, x: f32, y: f32) -> Self {
        Self {
            object: Some(object),
            x,
            y,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InstanceLayer {
    #[serde(rename = "%Name")]
    pub display_name: String,

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

    pub instances: Vec<Instance>,
    pub layers: Vec<Layer>,

    pub name: String,
    pub properties: Vec<Value>,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    #[serde(rename = "userdefinedDepth")]
    pub user_defined_depth: bool,
    pub visible: bool,
}

impl Default for InstanceLayer {
    fn default() -> Self {
        Self {
            display_name: String::new(),
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

            name: String::new(),
            properties: Vec::new(),

            resource_type: "GMRInstanceLayer".to_string(),
            resource_version: "2.0".to_string(),
            user_defined_depth: false,
            visible: true,
        }
    }
}

impl InstanceLayer {
    pub fn new(name: impl Into<String>, depth: i32) -> Self {
        let name = name.into();

        Self {
            display_name: name.clone(),
            name,
            depth,
            ..Default::default()
        }
    }

    pub fn add_instance(&mut self, instance: Instance) {
        self.instances.push(instance);
    }

    pub fn with_instance(mut self, instance: Instance) -> Self {
        self.add_instance(instance);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BackgroundLayer {
    #[serde(rename = "%Name")]
    pub display_name: String,

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

    pub name: String,
    pub layers: Vec<Layer>,
    pub properties: Vec<Value>,

    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

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

impl Default for BackgroundLayer {
    fn default() -> Self {
        Self {
            display_name: String::new(),

            animation_fps: 15.0,
            animation_speed_type: 0,

            colour: 0xFFFFFFFF,
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
            name: String::new(),
            properties: Vec::new(),

            resource_type: "GMRBackgroundLayer".to_string(),
            resource_version: "2.0".to_string(),

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

impl BackgroundLayer {
    pub fn new(name: impl Into<String>, depth: i32) -> Self {
        let name = name.into();

        Self {
            display_name: name.clone(),
            name,
            depth,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Layer {
    Instance(InstanceLayer),
    Background(BackgroundLayer),
}

pub trait LayerTrait {
    fn name(&self) -> &str;
    fn instances(&self) -> Option<&Vec<Instance>>;
}

impl LayerTrait for Layer {
    fn name(&self) -> &str {
        match self {
            Layer::Instance(layer) => &layer.name,
            Layer::Background(layer) => &layer.name,
        }
    }

    fn instances(&self) -> Option<&Vec<Instance>> {
        match self {
            Layer::Instance(layer) => Some(&layer.instances),
            Layer::Background(_) => None,
        }
    }
}

impl Default for Layer {
    fn default() -> Self {
        Layer::Instance(InstanceLayer::default())
    }
}

impl Layer {
    pub fn instance_layer(name: impl Into<String>, depth: i32) -> Self {
        Self::Instance(InstanceLayer::new(name, depth))
    }

    pub fn background_layer(name: impl Into<String>, depth: i32) -> Self {
        Self::Background(BackgroundLayer::new(name, depth))
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
                let layer = serde_json::from_value::<InstanceLayer>(value)
                    .map_err(serde::de::Error::custom)?;

                Ok(Layer::Instance(layer))
            }

            None => match value.get("$GMRBackgroundLayer") {
                Some(_) => {
                    let layer = serde_json::from_value::<BackgroundLayer>(value)
                        .map_err(serde::de::Error::custom)?;

                    Ok(Layer::Background(layer))
                }

                None => Err(serde::de::Error::custom("unknown GameMaker layer type")),
            },
        }
    }
}
