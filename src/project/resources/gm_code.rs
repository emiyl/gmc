use crate::project::resources::event_type::{EventSubType, EventType};
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CodeEntry {
    pub owner: CodeOwner,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub enum CodeOwner {
    Script {
        name: String,
    },
    ObjectEvent {
        object: String,
        event_type: EventType,
        event_num: EventSubType,
    },
}

impl CodeEntry {
    pub fn new_script(name: &str, code: &str) -> Self {
        CodeEntry {
            owner: CodeOwner::Script {
                name: name.to_string(),
            },
            name: name.to_string(),
            code: code.to_string(),
        }
    }

    pub fn new_object_event(
        object: &str,
        event_type: EventType,
        event_num: EventSubType,
        code: &str,
    ) -> Self {
        let name = format!(
            "gml_Object_{}_{}_{}",
            object,
            event_type.as_str(),
            event_num.value()
        );
        CodeEntry {
            owner: CodeOwner::ObjectEvent {
                object: object.to_string(),
                event_type,
                event_num: event_num,
            },
            name,
            code: code.to_string(),
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        // if path is in objects/<object_name>/<EventType>_<EventSubType>.gml, parse it as such
        let path = path.as_ref();
        let file_name = path.file_name().ok_or("Invalid path")?.to_string_lossy();
        let parent = path.parent().ok_or("Invalid path")?;
        let grandparent = parent.parent().ok_or("Invalid path")?;

        if grandparent.file_name().map(|s| s.to_string_lossy()) == Some("objects".into()) {
            let object_name = parent.file_name().ok_or("Invalid path")?.to_string_lossy();
            let event_parts: Vec<&str> = file_name
                .split('_')
                .into_iter()
                .map(|s| s.trim_end_matches(".gml"))
                .collect();
            if event_parts.len() != 2 {
                return Err("Invalid event file name".into());
            }
            let event_type =
                EventType::from_str(event_parts[0]).map_err(|_| "Invalid event type")?;
            let event_num = EventSubType::from_i32(event_type, event_parts[1].parse()?);
            let mut file = std::fs::File::open(path)?;
            let mut code = String::new();
            file.read_to_string(&mut code)?;
            Ok(CodeEntry::new_object_event(
                &object_name,
                event_type,
                event_num,
                &code,
            ))
        } else {
            // otherwise, treat it as a script
            let mut file = std::fs::File::open(path)?;
            let mut code = String::new();
            file.read_to_string(&mut code)?;
            Ok(CodeEntry::new_script(&file_name, &code))
        }
    }

    pub fn save<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        std::fs::create_dir_all(path.parent().ok_or("Invalid path")?)?;
        std::fs::write(path, &self.code)?;
        Ok(())
    }
}
