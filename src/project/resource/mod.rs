mod object;
mod path;
pub mod room;
mod script;
mod sequence;
mod shader;
mod sprite;
mod track;

use super::ResourceId;
use crate::project::formatter::read_gamemaker_json;

pub use object::GMObject;
pub use path::GMPath;
pub use room::GMRoom;
pub use script::GMScript;
pub use shader::GMShader;
pub use sprite::GMSprite;

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceKind {
    Room,
    Object,
    Script,
    Sprite,
    Shader,
    Path,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Room(GMRoom),
    Object(GMObject),
    Script(GMScript),
    Sprite(GMSprite),
    Shader(GMShader),
    Path(GMPath),
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
            Resource::Sprite(sprite) => sprite.name(),
            Resource::Shader(shader) => shader.name(),
            Resource::Path(path) => path.name(),
        }
    }
    fn save(&self, file_path: &std::path::Path) -> std::io::Result<()> {
        let parent_dir = file_path.parent().expect("Failed to get parent directory");
        fs::create_dir_all(parent_dir)?;
        match self {
            Resource::Room(room) => room.save(file_path),
            Resource::Object(object) => object.save(file_path),
            Resource::Script(script) => script.save(file_path),
            Resource::Sprite(sprite) => sprite.save(file_path),
            Resource::Shader(shader) => shader.save(file_path),
            Resource::Path(path) => path.save(file_path),
        }
    }
    fn default_path(&self) -> String {
        match self {
            Resource::Room(room) => room.default_path(),
            Resource::Object(object) => object.default_path(),
            Resource::Script(script) => script.default_path(),
            Resource::Sprite(sprite) => sprite.default_path(),
            Resource::Shader(shader) => shader.default_path(),
            Resource::Path(path) => path.default_path(),
        }
    }
}

impl Resource {
    pub fn new(name: &str, kind: ResourceKind, parent: ResourceId) -> Self {
        match kind {
            ResourceKind::Room => {
                let room = GMRoom::new(name, parent);
                Resource::from_room(room)
            }
            ResourceKind::Object => {
                let object = GMObject::new(name, parent);
                Resource::from_object(object)
            }
            ResourceKind::Script => {
                let script = GMScript::new(name, parent);
                Resource::from_script(script)
            }
            ResourceKind::Sprite => {
                let sprite = GMSprite::new(
                    name,
                    parent,
                    &std::path::Path::new(&format!("sprites/{}/{}.yy", name, name)),
                );
                Resource::from_sprite(sprite)
            }
            ResourceKind::Shader => {
                let shader = GMShader::new(name, parent);
                Resource::from_shader(shader)
            }
            ResourceKind::Path => {
                let path = GMPath::new(name, parent);
                Resource::from_path(path)
            }
        }
    }
    pub fn load(path: &std::path::Path) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let value = read_gamemaker_json(path)?;

        match value.get("resourceType").and_then(|v| v.as_str()) {
            Some("GMRoom") => {
                let room = GMRoom::load(value)?;
                Ok(Resource::Room(room))
            }
            Some("GMObject") => {
                let object = GMObject::load(value)?;
                Ok(Resource::Object(object))
            }
            Some("GMScript") => {
                let script = GMScript::load(value)?;
                Ok(Resource::Script(script))
            }
            Some("GMSprite") => {
                let sprite = GMSprite::load(value)?;
                Ok(Resource::Sprite(sprite))
            }
            Some("GMShader") => {
                let shader = GMShader::load(value)?;
                Ok(Resource::Shader(shader))
            }
            _ => {
                unimplemented!("Loading for this resource type is not implemented yet")
            }
        }
    }

    pub fn kind(&self) -> ResourceKind {
        match self {
            Resource::Room(_) => ResourceKind::Room,
            Resource::Object(_) => ResourceKind::Object,
            Resource::Script(_) => ResourceKind::Script,
            Resource::Sprite(_) => ResourceKind::Sprite,
            Resource::Shader(_) => ResourceKind::Shader,
            Resource::Path(_) => ResourceKind::Path,
        }
    }

    pub fn as_room(&self) -> Option<&GMRoom> {
        match self {
            Resource::Room(room) => Some(room),
            _ => None,
        }
    }
    pub fn as_room_mut(&mut self) -> Option<&mut GMRoom> {
        match self {
            Resource::Room(room) => Some(room),
            _ => None,
        }
    }
    pub fn from_room(room: GMRoom) -> Self {
        Resource::Room(room)
    }

    pub fn as_object(&self) -> Option<&GMObject> {
        match self {
            Resource::Object(object) => Some(object),
            _ => None,
        }
    }
    pub fn as_object_mut(&mut self) -> Option<&mut GMObject> {
        match self {
            Resource::Object(object) => Some(object),
            _ => None,
        }
    }
    pub fn from_object(object: GMObject) -> Self {
        Resource::Object(object)
    }

    pub fn as_script(&self) -> Option<&GMScript> {
        match self {
            Resource::Script(script) => Some(script),
            _ => None,
        }
    }
    pub fn as_script_mut(&mut self) -> Option<&mut GMScript> {
        match self {
            Resource::Script(script) => Some(script),
            _ => None,
        }
    }
    pub fn from_script(script: GMScript) -> Self {
        Resource::Script(script)
    }

    pub fn as_sprite(&self) -> Option<&GMSprite> {
        match self {
            Resource::Sprite(sprite) => Some(sprite),
            _ => None,
        }
    }
    pub fn as_sprite_mut(&mut self) -> Option<&mut GMSprite> {
        match self {
            Resource::Sprite(sprite) => Some(sprite),
            _ => None,
        }
    }
    pub fn from_sprite(sprite: GMSprite) -> Self {
        Resource::Sprite(sprite)
    }

    pub fn as_shader(&self) -> Option<&GMShader> {
        match self {
            Resource::Shader(shader) => Some(shader),
            _ => None,
        }
    }
    pub fn as_shader_mut(&mut self) -> Option<&mut GMShader> {
        match self {
            Resource::Shader(shader) => Some(shader),
            _ => None,
        }
    }
    pub fn from_shader(shader: GMShader) -> Self {
        Resource::Shader(shader)
    }

    pub fn as_path(&self) -> Option<&GMPath> {
        match self {
            Resource::Path(path) => Some(path),
            _ => None,
        }
    }
    pub fn as_path_mut(&mut self) -> Option<&mut GMPath> {
        match self {
            Resource::Path(path) => Some(path),
            _ => None,
        }
    }
    pub fn from_path(path: GMPath) -> Self {
        Resource::Path(path)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceType {
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

impl Default for ResourceType {
    fn default() -> Self {
        Self {
            resource_type: "Resource".to_string(),
            resource_version: "2.0".to_string(),
        }
    }
}

impl ResourceType {
    fn new(resource_type: &str) -> Self {
        Self {
            resource_type: resource_type.to_string(),
            resource_version: "2.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceBase {
    #[serde(rename = "%Name")]
    pub display_name: String,

    pub name: String,

    #[serde(flatten)]
    pub resource_type: ResourceType,
}

impl Default for ResourceBase {
    fn default() -> Self {
        Self {
            display_name: "Resource1".to_string(),
            name: "Resource1".to_string(),
            resource_type: ResourceType::default(),
        }
    }
}

impl ResourceBase {
    fn new(name: &str, resource_type: &str) -> Self {
        Self {
            display_name: name.to_string(),
            name: name.to_string(),
            resource_type: ResourceType::new(resource_type),
        }
    }
}
