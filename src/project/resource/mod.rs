mod object;
pub mod room;

use crate::project::formatter::read_gamemaker_json;

use super::ResourceId;
pub use object::Object;
pub use room::Room;

use serde_json::Value;
use std::fs;
use std::io::Read;

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceKind {
    Room,
    Object,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Room(Room),
    Object(Object),
}

pub trait ResourceTrait {
    fn name(&self) -> &str;
    fn save(&self, path: &std::path::Path) -> std::io::Result<()>;
    fn default_path(&self) -> String;
}

impl ResourceTrait for Resource {
    fn name(&self) -> &str {
        match self {
            Resource::Room(room) => room.name(),
            Resource::Object(object) => object.name(),
        }
    }
    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let parent_dir = path.parent().expect("Failed to get parent directory");
        fs::create_dir_all(parent_dir)?;
        match self {
            Resource::Room(room) => room.save(path),
            Resource::Object(object) => object.save(path),
        }
    }
    fn default_path(&self) -> String {
        match self {
            Resource::Room(room) => room.default_path(),
            Resource::Object(object) => object.default_path(),
        }
    }
}

impl Resource {
    pub fn new(name: &str, kind: ResourceKind, parent: ResourceId) -> Self {
        match kind {
            ResourceKind::Room => {
                let room = Room::new(name, parent);
                Resource::from_room(room)
            }
            ResourceKind::Object => {
                let object = Object::new(name, parent);
                Resource::from_object(object)
            }
        }
    }
    pub fn load(path: &std::path::Path) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let value = read_gamemaker_json(path)?;

        if value.get("$GMRoom").is_some() {
            let room = Room::load(value)?;
            Ok(Resource::Room(room))
        } else if value.get("$GMObject").is_some() {
            let object = Object::load(value)?;
            Ok(Resource::Object(object))
        } else {
            unimplemented!("Loading for this resource type is not implemented yet")
        }
    }

    pub fn kind(&self) -> ResourceKind {
        match self {
            Resource::Room(_) => ResourceKind::Room,
            Resource::Object(_) => ResourceKind::Object,
        }
    }

    pub fn as_room(&self) -> Option<&Room> {
        match self {
            Resource::Room(room) => Some(room),
            _ => None,
        }
    }
    pub fn as_room_mut(&mut self) -> Option<&mut Room> {
        match self {
            Resource::Room(room) => Some(room),
            _ => None,
        }
    }
    pub fn from_room(room: Room) -> Self {
        Resource::Room(room)
    }

    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Resource::Object(object) => Some(object),
            _ => None,
        }
    }
    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Resource::Object(object) => Some(object),
            _ => None,
        }
    }
    pub fn from_object(object: Object) -> Self {
        Resource::Object(object)
    }
}
