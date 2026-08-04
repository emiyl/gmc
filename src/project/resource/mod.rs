mod object;
pub mod room;
mod script;
mod sprite;

use super::ResourceId;
use crate::project::formatter::read_gamemaker_json;

pub use object::Object;
pub use room::Room;
pub use script::Script;
pub use sprite::Sprite;

use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceKind {
    Room,
    Object,
    Script,
    Sprite,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Room(Room),
    Object(Object),
    Script(Script),
    Sprite(Sprite),
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
            Resource::Script(script) => script.name(),
        }
    }
    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let parent_dir = path.parent().expect("Failed to get parent directory");
        fs::create_dir_all(parent_dir)?;
        match self {
            Resource::Room(room) => room.save(path),
            Resource::Object(object) => object.save(path),
            Resource::Script(script) => script.save(path),
        }
    }
    fn default_path(&self) -> String {
        match self {
            Resource::Room(room) => room.default_path(),
            Resource::Object(object) => object.default_path(),
            Resource::Script(script) => script.default_path(),
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
            ResourceKind::Script => {
                let script = Script::new(name, parent);
                Resource::from_script(script)
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
        } else if value.get("$GMScript").is_some() {
            let script = Script::load(value)?;
            Ok(Resource::Script(script))
        } else {
            unimplemented!("Loading for this resource type is not implemented yet")
        }
    }

    pub fn kind(&self) -> ResourceKind {
        match self {
            Resource::Room(_) => ResourceKind::Room,
            Resource::Object(_) => ResourceKind::Object,
            Resource::Script(_) => ResourceKind::Script,
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

    pub fn as_script(&self) -> Option<&Script> {
        match self {
            Resource::Script(script) => Some(script),
            _ => None,
        }
    }
    pub fn as_script_mut(&mut self) -> Option<&mut Script> {
        match self {
            Resource::Script(script) => Some(script),
            _ => None,
        }
    }
    pub fn from_script(script: Script) -> Self {
        Resource::Script(script)
    }
}
