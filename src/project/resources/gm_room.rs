//! Parser and data model for GameMaker `.yy` room files (e.g. `Room1.yy`).
//!
//! `.yy` files are JSON, but GameMaker's own writer leaves trailing commas
//! before closing `}` / `]`, which is not valid strict JSON. [`parse_str`]
//! and [`parse_file`] strip those before handing the text to `serde_json`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

use super::{Resource, ResourceRef};

// ---------------------------------------------------------------------
// Top-level room
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GmRoom {
    #[serde(rename = "$GMRoom")]
    pub gm_room: String,

    #[serde(rename = "%Name")]
    pub percent_name: String,

    #[serde(rename = "creationCodeFile")]
    pub creation_code_file: String,

    #[serde(rename = "inheritCode")]
    pub inherit_code: bool,

    #[serde(rename = "inheritCreationOrder")]
    pub inherit_creation_order: bool,

    #[serde(rename = "inheritLayers")]
    pub inherit_layers: bool,

    #[serde(rename = "instanceCreationOrder")]
    pub instance_creation_order: Vec<Value>,

    #[serde(rename = "isDnd")]
    pub is_dnd: bool,

    pub layers: Vec<Layer>,

    pub name: String,

    pub parent: ResourceRef,

    #[serde(rename = "parentRoom")]
    pub parent_room: Option<Value>,

    #[serde(rename = "physicsSettings")]
    pub physics_settings: PhysicsSettings,

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,

    #[serde(rename = "roomSettings")]
    pub room_settings: RoomSettings,

    #[serde(rename = "sequenceId")]
    pub sequence_id: Option<Value>,

    pub views: Vec<View>,

    #[serde(rename = "viewSettings")]
    pub view_settings: ViewSettings,

    pub volume: f64,
}

impl Default for GmRoom {
    fn default() -> Self {
        GmRoom {
            gm_room: "v1".to_string(),
            percent_name: "Room1".to_string(),
            creation_code_file: String::new(),
            inherit_code: false,
            inherit_creation_order: false,
            inherit_layers: false,
            instance_creation_order: Vec::new(),
            is_dnd: false,
            layers: vec![Layer::instances_default(), Layer::background_default()],
            name: "Room1".to_string(),
            parent: ResourceRef {
                name: "BLANK GAME".to_string(),
                path: "BLANK GAME.yyp".to_string(),
            },
            parent_room: None,
            physics_settings: PhysicsSettings::default(),
            resource_type: "GMRoom".to_string(),
            resource_version: "2.0".to_string(),
            room_settings: RoomSettings::default(),
            sequence_id: None,
            views: (0..8).map(|_| View::default()).collect(),
            view_settings: ViewSettings::default(),
            volume: 1.0,
        }
    }
}

impl Resource for GmRoom {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_path(&self) -> &str {
        let path = &self.parent.path;
        println!("Parent path: {}", path);
        // &format!("rooms/{}/{}.yy", self.name, self.name)
        path
    }
}

impl GmRoom {
    pub fn new(name: &str, parent: ResourceRef) -> Self {
        GmRoom {
            name: name.to_string(),
            percent_name: name.to_string(),
            parent,
            ..GmRoom::default()
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, YyError> {
        let contents = fs::read_to_string(path)?;
        parse_str(&contents)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), YyError> {
        let json = to_pretty_string(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------
// Physics / room / view settings
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicsSettings {
    #[serde(rename = "inheritPhysicsSettings")]
    pub inherit_physics_settings: bool,
    #[serde(rename = "PhysicsWorld")]
    pub physics_world: bool,
    #[serde(rename = "PhysicsWorldGravityX")]
    pub physics_world_gravity_x: f64,
    #[serde(rename = "PhysicsWorldGravityY")]
    pub physics_world_gravity_y: f64,
    #[serde(rename = "PhysicsWorldPixToMetres")]
    pub physics_world_pix_to_metres: f64,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        PhysicsSettings {
            inherit_physics_settings: false,
            physics_world: false,
            physics_world_gravity_x: 0.0,
            physics_world_gravity_y: 10.0,
            physics_world_pix_to_metres: 0.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomSettings {
    #[serde(rename = "Height")]
    pub height: i64,
    #[serde(rename = "inheritRoomSettings")]
    pub inherit_room_settings: bool,
    pub persistent: bool,
    #[serde(rename = "Width")]
    pub width: i64,
}

impl Default for RoomSettings {
    fn default() -> Self {
        RoomSettings {
            height: 768,
            inherit_room_settings: false,
            persistent: false,
            width: 1366,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewSettings {
    #[serde(rename = "clearDisplayBuffer")]
    pub clear_display_buffer: bool,
    #[serde(rename = "clearViewBackground")]
    pub clear_view_background: bool,
    #[serde(rename = "enableViews")]
    pub enable_views: bool,
    #[serde(rename = "inheritViewSettings")]
    pub inherit_view_settings: bool,
}

impl Default for ViewSettings {
    fn default() -> Self {
        ViewSettings {
            clear_display_buffer: true,
            clear_view_background: false,
            enable_views: false,
            inherit_view_settings: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    pub hborder: i64,
    pub hport: i64,
    pub hspeed: i64,
    pub hview: i64,
    pub inherit: bool,
    #[serde(rename = "objectId")]
    pub object_id: Option<Value>,
    pub vborder: i64,
    pub visible: bool,
    pub vspeed: i64,
    pub wport: i64,
    pub wview: i64,
    pub xport: i64,
    pub xview: i64,
    pub yport: i64,
    pub yview: i64,
}

impl Default for View {
    fn default() -> Self {
        View {
            hborder: 32,
            hport: 768,
            hspeed: -1,
            hview: 768,
            inherit: false,
            object_id: None,
            vborder: 32,
            visible: false,
            vspeed: -1,
            wport: 1366,
            wview: 1366,
            xport: 0,
            xview: 0,
            yport: 0,
            yview: 0,
        }
    }
}

// ---------------------------------------------------------------------
// Layers: an "Instances" layer and a "Background" layer share a common
// set of fields; each also carries its own marker key and extra fields.
// Modelled as an internally-tagged enum keyed on "resourceType" so a
// Vec<Layer> round-trips both kinds (and any nested sub-layers).
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resourceType")]
pub enum Layer {
    #[serde(rename = "GMRInstanceLayer")]
    Instance(InstanceLayer),
    #[serde(rename = "GMRBackgroundLayer")]
    Background(BackgroundLayer),
}

impl Layer {
    pub fn instances_default() -> Self {
        Layer::Instance(InstanceLayer::default())
    }

    pub fn background_default() -> Self {
        Layer::Background(BackgroundLayer::default())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceLayer {
    #[serde(rename = "$GMRInstanceLayer")]
    pub gm_instance_layer: String,
    #[serde(rename = "%Name")]
    pub percent_name: String,
    pub depth: i64,
    #[serde(rename = "effectEnabled")]
    pub effect_enabled: bool,
    #[serde(rename = "effectType")]
    pub effect_type: Option<Value>,
    #[serde(rename = "gridX")]
    pub grid_x: i64,
    #[serde(rename = "gridY")]
    pub grid_y: i64,
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
    pub instances: Vec<Value>,
    pub layers: Vec<Layer>,
    pub name: String,
    pub properties: Vec<Value>,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(rename = "userdefinedDepth")]
    pub userdefined_depth: bool,
    pub visible: bool,
}

impl Default for InstanceLayer {
    fn default() -> Self {
        InstanceLayer {
            gm_instance_layer: String::new(),
            percent_name: "Instances".to_string(),
            depth: 0,
            effect_enabled: true,
            effect_type: None,
            grid_x: 32,
            grid_y: 32,
            hierarchy_frozen: false,
            inherit_layer_depth: false,
            inherit_layer_settings: false,
            inherit_sub_layers: true,
            inherit_visibility: true,
            instances: Vec::new(),
            layers: Vec::new(),
            name: "Instances".to_string(),
            properties: Vec::new(),
            resource_version: "2.0".to_string(),
            userdefined_depth: false,
            visible: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundLayer {
    #[serde(rename = "$GMRBackgroundLayer")]
    pub gm_background_layer: String,
    #[serde(rename = "%Name")]
    pub percent_name: String,
    #[serde(rename = "animationFPS")]
    pub animation_fps: f64,
    #[serde(rename = "animationSpeedType")]
    pub animation_speed_type: i64,
    pub colour: i64,
    pub depth: i64,
    #[serde(rename = "effectEnabled")]
    pub effect_enabled: bool,
    #[serde(rename = "effectType")]
    pub effect_type: Option<Value>,
    #[serde(rename = "gridX")]
    pub grid_x: i64,
    #[serde(rename = "gridY")]
    pub grid_y: i64,
    #[serde(rename = "hierarchyFrozen")]
    pub hierarchy_frozen: bool,
    pub hspeed: f64,
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
    pub name: String,
    pub properties: Vec<Value>,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(rename = "spriteId")]
    pub sprite_id: Option<Value>,
    pub stretch: bool,
    #[serde(rename = "userdefinedAnimFPS")]
    pub userdefined_anim_fps: bool,
    #[serde(rename = "userdefinedDepth")]
    pub userdefined_depth: bool,
    pub visible: bool,
    pub vspeed: f64,
    pub vtiled: bool,
    pub x: i64,
    pub y: i64,
}

impl Default for BackgroundLayer {
    fn default() -> Self {
        BackgroundLayer {
            gm_background_layer: String::new(),
            percent_name: "Background".to_string(),
            animation_fps: 15.0,
            animation_speed_type: 0,
            colour: 4278190080,
            depth: 100,
            effect_enabled: true,
            effect_type: None,
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
            name: "Background".to_string(),
            properties: Vec::new(),
            resource_version: "2.0".to_string(),
            sprite_id: None,
            stretch: false,
            userdefined_anim_fps: false,
            userdefined_depth: false,
            visible: true,
            vspeed: 0.0,
            vtiled: false,
            x: 0,
            y: 0,
        }
    }
}

// ---------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------

#[derive(Debug)]
pub enum YyError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for YyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YyError::Io(e) => write!(f, "I/O error reading .yy file: {e}"),
            YyError::Json(e) => write!(f, "JSON error parsing .yy file: {e}"),
        }
    }
}

impl std::error::Error for YyError {}

impl From<std::io::Error> for YyError {
    fn from(e: std::io::Error) -> Self {
        YyError::Io(e)
    }
}

impl From<serde_json::Error> for YyError {
    fn from(e: serde_json::Error) -> Self {
        YyError::Json(e)
    }
}

/// Removes trailing commas that appear right before a closing `}` or `]`.
///
/// GameMaker's own serializer writes these (see the sample `Room1.yy`),
/// but they are not valid JSON, so `serde_json` rejects them as-is.
/// This walks the raw text once, tracking whether we're inside a quoted
/// string (respecting `\"` escapes) so commas inside string values are
/// left untouched.
pub fn strip_trailing_commas(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len());
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i] as char;

        if in_string {
            out.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if c == '"' {
            in_string = true;
            out.push(c);
            i += 1;
            continue;
        }

        if c == ',' {
            // Look ahead past whitespace to see if a closer follows.
            let mut j = i + 1;
            while j < bytes.len() && (bytes[j] as char).is_whitespace() {
                j += 1;
            }
            if j < bytes.len() && (bytes[j] == b'}' || bytes[j] == b']') {
                // Drop the comma; keep scanning from the closer.
                i += 1;
                continue;
            }
        }

        out.push(c);
        i += 1;
    }

    out
}

/// Parses `.yy` room JSON text (tolerating GameMaker's trailing commas).
pub fn parse_str(contents: &str) -> Result<GmRoom, YyError> {
    let cleaned = strip_trailing_commas(contents);
    let room: GmRoom = serde_json::from_str(&cleaned)?;
    Ok(room)
}

/// Reads and parses a `.yy` room file from disk.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<GmRoom, YyError> {
    let contents = fs::read_to_string(path)?;
    parse_str(&contents)
}

/// Serializes a room back to pretty-printed, strict JSON (no trailing
/// commas). GameMaker's IDE accepts standard JSON on import, so this is
/// safe to write straight back to a `.yy` file.
pub fn to_pretty_string(room: &GmRoom) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(room)
}

/// Serializes and writes a room to a `.yy` file on disk.
pub fn write_file<P: AsRef<Path>>(path: P, room: &GmRoom) -> Result<(), YyError> {
    let json = to_pretty_string(room)?;
    fs::write(path, json)?;
    Ok(())
}
