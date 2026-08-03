use crate::project::ResourceId;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Event {
    #[serde(rename = "$GMEvent")]
    pub gm_event: String,

    #[serde(rename = "%Name")]
    pub display_name_internal: String,

    #[serde(rename = "collisionObjectId")]
    pub collision_object_id: Option<ResourceId>,

    #[serde(rename = "eventNum")]
    pub event_num: i32,

    #[serde(rename = "eventType")]
    pub event_type: i32,

    #[serde(rename = "isDnD")]
    pub is_dnd: bool,

    pub name: String,

    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            gm_event: "v1".into(),
            display_name_internal: "".into(),
            collision_object_id: None,
            event_num: (&EventSubType::None).value(),
            event_type: EventType::Create as i32,
            is_dnd: false,
            name: "".into(),
            resource_type: "GMEvent".into(),
            resource_version: "2.0".into(),
        }
    }
}

impl Event {
    pub fn new(event_type: String, event_subtype: Option<String>) -> Self {
        let event_type_enum = EventType::from_str(&event_type).unwrap_or(EventType::Create);
        let event_subtype_enum = match event_subtype {
            Some(subtype) => EventSubType::from_str(event_type_enum, &subtype),
            None => None,
        };

        let event_type = event_type_enum as i32;
        let event_num = match event_subtype_enum {
            Some(subtype) => subtype.value(),
            None => 0,
        };

        Self {
            event_type,
            event_num,
            ..Default::default()
        }
    }

    pub fn get_code_path_from_object_path(
        &self,
        object_path: &std::path::Path,
    ) -> std::path::PathBuf {
        // path here is the path to the object file e.g. "objects/MyObject/MyObject.yy"
        // we need objects/MyObject/<EventType>_<EventSubType>.gml
        object_path.with_file_name(format!(
            "{}_{}.gml",
            EventType::try_from(self.event_type).unwrap_or(EventType::Create),
            self.event_num
        ))
    }

    pub fn ensure_code_file_exists(&self, path: &std::path::Path) -> std::io::Result<()> {
        let path = self.get_code_path_from_object_path(path);
        if !path.exists() {
            let file = std::fs::File::create(path)?;
            file.set_len(0)?; // create an empty file
        }
        Ok(())
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Display, EnumString, IntoPrimitive, TryFromPrimitive)]
#[strum(ascii_case_insensitive)]
pub enum EventType {
    Create = 0,
    Destroy = 1,
    Alarm = 2,
    Step = 3,
    Collision = 4,
    Keyboard = 5,
    Mouse = 6,
    Other = 7,
    Draw = 8,
    KeyPress = 9,
    KeyRelease = 10,
    Cleanup = 12,
    PreCreate = 14,
}

#[derive(Debug, Clone)]
pub enum EventSubType {
    None,
    Alarm(u32),
    Step(StepEvent),
    Draw(DrawEvent),
    Mouse(MouseEvent),
    Other(OtherEvent),
    Keyboard(u32),
    KeyPress(u32),
    KeyRelease(u32),
    Collision(String),
}

impl EventSubType {
    pub fn value(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Alarm(n) => *n as i32,
            Self::Step(e) => (*e).into(),
            Self::Draw(e) => (*e).into(),
            Self::Mouse(e) => (*e).into(),
            Self::Other(e) => (*e).into(),
            Self::Keyboard(n) => *n as i32,
            Self::KeyPress(n) => *n as i32,
            Self::KeyRelease(n) => *n as i32,
            Self::Collision(_) => 0,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::None => "None".into(),
            Self::Alarm(n) => format!("{n}"),
            Self::Step(e) => e.to_string(),
            Self::Draw(e) => e.to_string(),
            Self::Mouse(e) => e.to_string(),
            Self::Other(e) => e.to_string(),
            Self::Keyboard(k) => format!("{k}"),
            Self::KeyPress(k) => format!("{k}"),
            Self::KeyRelease(k) => format!("{k}"),
            Self::Collision(name) => name.clone(),
        }
    }
}

impl EventSubType {
    pub fn from_str(event_type: EventType, s: &str) -> Option<Self> {
        Some(match event_type {
            EventType::Step => Self::Step(s.parse().ok()?),
            EventType::Draw => Self::Draw(s.parse().ok()?),
            EventType::Mouse => Self::Mouse(s.parse().ok()?),
            EventType::Other => Self::Other(s.parse().ok()?),

            EventType::Alarm => Self::Alarm(s.parse().ok()?),
            EventType::Keyboard => Self::Keyboard(s.parse().ok()?),
            EventType::KeyPress => Self::KeyPress(s.parse().ok()?),
            EventType::KeyRelease => Self::KeyRelease(s.parse().ok()?),

            EventType::Collision => Self::Collision(s.to_owned()),

            _ => Self::None,
        })
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Display, EnumString, IntoPrimitive, TryFromPrimitive)]
#[strum(ascii_case_insensitive)]
pub enum StepEvent {
    Normal = 0,
    Begin = 1,
    End = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Display, EnumString, IntoPrimitive, TryFromPrimitive)]
#[strum(ascii_case_insensitive)]
pub enum DrawEvent {
    Normal = 0,
    Gui = 64,
    Begin = 72,
    End = 73,
    GuiBegin = 74,
    GuiEnd = 75,
    Pre = 76,
    Post = 77,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Display, EnumString, IntoPrimitive, TryFromPrimitive)]
#[strum(ascii_case_insensitive)]
pub enum MouseEvent {
    LeftButton = 0,
    RightButton = 1,
    MiddleButton = 2,
    // ...
    WheelUp = 60,
    WheelDown = 61,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Display, EnumString, IntoPrimitive, TryFromPrimitive)]
#[strum(ascii_case_insensitive)]
pub enum OtherEvent {
    OutsideRoom = 0,
    GameStart = 2,
    RoomStart = 4,
    // ...
    AsyncSystem = 75,
}
