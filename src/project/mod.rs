mod formatter;
mod gm_project;
mod options;
mod resource_order;
pub mod resources;

use gm_project::GmProjectYyp;
use options::Options;
use resource_order::ResourceOrder;
pub use resources::{
    CodeEntry, EventSubType, EventType, GmObject, GmRoom, ResourceRef, ResourceType,
};

pub use crate::project::resources::CodeOwner;

#[derive(Debug, Clone)]
pub struct GmProject {
    pub yyp: GmProjectYyp,
    pub resource_order: ResourceOrder,
    pub options: Options,
    pub rooms: Vec<GmRoom>,
    pub objects: Vec<GmObject>,
    pub code: Vec<CodeEntry>,
}

impl GmProject {
    pub fn new(name: &str) -> Self {
        GmProject {
            yyp: GmProjectYyp::new(name),
            resource_order: ResourceOrder::new(),
            options: Options::new(name),
            rooms: Vec::new(),
            objects: Vec::new(),
            code: Vec::new(),
        }
    }

    pub fn save(&self, project_path: &std::path::Path) -> std::io::Result<()> {
        let parent_dir = project_path
            .parent()
            .expect("Failed to get parent directory of project path");
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let datafiles_path = parent_dir.join("datafiles");
        if !datafiles_path.exists() {
            std::fs::create_dir_all(&datafiles_path)?;
        }

        let options_path = parent_dir.join("options");
        self.options
            .save(&options_path)
            .expect("Failed to save options");

        self.yyp
            .save(&project_path)
            .expect("Failed to save project file");

        let resource_order_path = parent_dir.join(format!("{}.resource_order", self.yyp.name));
        self.resource_order
            .save(&resource_order_path)
            .expect("Failed to save resource order");

        let rooms_dir = parent_dir.join("rooms");
        let rooms_vec = &self.rooms;
        if !rooms_vec.is_empty() {
            if !rooms_dir.exists() {
                std::fs::create_dir_all(&rooms_dir)?;
            }
            for room in rooms_vec {
                let room_dir = rooms_dir.join(&room.name);
                if !room_dir.exists() {
                    std::fs::create_dir_all(&room_dir)?;
                }
                let room_file_path = room_dir.join(format!("{}.yy", room.name));
                room.save(&room_file_path)
                    .expect("Failed to save room file");
            }
        }

        let objects_dir = parent_dir.join("objects");
        let objects_vec = &self.objects;
        if !objects_vec.is_empty() {
            if !objects_dir.exists() {
                std::fs::create_dir_all(&objects_dir)?;
            }
            for object in objects_vec {
                let object_dir = objects_dir.join(&object.name);
                if !object_dir.exists() {
                    std::fs::create_dir_all(&object_dir)?;
                }
                let object_file_path = object_dir.join(format!("{}.yy", object.name));
                object
                    .save(&object_file_path)
                    .expect("Failed to save object file");
            }
        }

        let code_vec = &self.code;
        if !code_vec.is_empty() {
            for code_entry in code_vec {
                let code_file_path = match &code_entry.owner {
                    CodeOwner::ObjectEvent {
                        object,
                        event_type,
                        event_num,
                    } => {
                        let object_dir = objects_dir.join(object);
                        if !object_dir.exists() {
                            std::fs::create_dir_all(&object_dir)?;
                        }
                        object_dir.join(format!(
                            "{}_{}.gml",
                            event_type.as_str(),
                            event_num.value()
                        ))
                    }
                    CodeOwner::Script { name: _ } => parent_dir.join("scripts"),
                };
                if !code_file_path.parent().unwrap().exists() {
                    std::fs::create_dir_all(code_file_path.parent().unwrap())?;
                }
                println!("Saving code entry to: {:?}", code_file_path);
                code_entry
                    .save(&code_file_path)
                    .expect("Failed to save code entry");
            }
        }

        Ok(())
    }

    pub fn load(project_file_path: &std::path::Path) -> std::io::Result<Self> {
        let yyp = GmProjectYyp::load(&project_file_path).expect("Failed to load project file");

        let project_path = project_file_path
            .parent()
            .expect("Failed to get project directory");
        let project_name = project_file_path
            .file_stem()
            .expect("Failed to get project name")
            .to_string_lossy();

        let resource_order_path = project_path.join(format!("{}.resource_order", project_name));
        let resource_order =
            ResourceOrder::load(&resource_order_path).expect("Failed to load resource order");

        let options_path = project_path.join("options");
        let options = Options::load(&options_path).expect("Failed to load options");

        // load rooms
        // room dir is <project_path>/rooms
        let mut rooms_vec = Vec::new();
        let rooms_dir = project_path.join("rooms");
        if rooms_dir.exists() {
            for entry in std::fs::read_dir(&rooms_dir).expect("Failed to read rooms directory") {
                let entry = entry.expect("Failed to read room entry");
                let path = entry.path();

                // now check if the folder has a .yy file with the same name as the folder
                if path.is_dir() {
                    // Get the room name from the folder name
                    let room_name = path
                        .file_name()
                        .expect("Failed to get room folder name")
                        .to_string_lossy();
                    let room_file_path = path.join(format!("{}.yy", room_name));

                    if room_file_path.exists() {
                        let room = GmRoom::load(&room_file_path).expect("Failed to load room");
                        rooms_vec.push(room.clone());
                    }
                }
            }
        }

        // load objects
        let mut objects_vec = Vec::new();
        let objects_dir = project_path.join("objects");
        if objects_dir.exists() {
            for entry in std::fs::read_dir(&objects_dir).expect("Failed to read objects directory")
            {
                let entry = entry.expect("Failed to read object entry");
                let path = entry.path();

                // now check if the folder has a .yy file with the same name as the folder
                if path.is_dir() {
                    // Get the object name from the folder name
                    let object_name = path
                        .file_name()
                        .expect("Failed to get object folder name")
                        .to_string_lossy();
                    let object_file_path = path.join(format!("{}.yy", object_name));

                    if object_file_path.exists() {
                        let object =
                            GmObject::load(&object_file_path).expect("Failed to load object");
                        objects_vec.push(object.clone());
                    }
                }
            }
        }

        // "objects/<object_name>/<code_name>.gml"
        let mut code_vec = Vec::new();
        for object in &objects_vec {
            let object_dir = objects_dir.join(&object.name);
            if object_dir.exists() {
                for entry in
                    std::fs::read_dir(&object_dir).expect("Failed to read object directory")
                {
                    let entry = entry.expect("Failed to read code entry");
                    let path = entry.path();

                    if path.is_file() && path.extension().map_or(false, |ext| ext == "gml") {
                        let code_entry = CodeEntry::load(&path).expect("Failed to load code entry");
                        code_vec.push(code_entry);
                    }
                }
            }
        }

        Ok(GmProject {
            yyp,
            resource_order,
            options,
            rooms: rooms_vec,
            objects: objects_vec,
            code: code_vec,
        })
    }

    pub fn add_resource(&mut self, resource_type: ResourceType, name: &str) {
        match resource_type {
            ResourceType::Room => self.add_room(name),
            ResourceType::Object => self.add_object(name),
        }
    }

    fn add_room(&mut self, name: &str) {
        let parent = ResourceRef {
            name: self.yyp.name.clone(),
            path: format!("{}.yyp", self.yyp.name),
        };
        self.yyp.add_resource(
            ResourceType::Room,
            name,
            format!("rooms/{}/{}.yy", name, name).as_str(),
        );
        let room = GmRoom::new(name, parent);
        self.rooms.push(room);
    }

    fn add_object(&mut self, name: &str) {
        let parent = ResourceRef {
            name: self.yyp.name.clone(),
            path: format!("{}.yyp", self.yyp.name),
        };
        self.yyp.add_resource(
            ResourceType::Object,
            name,
            format!("objects/{}/{}.yy", name, name).as_str(),
        );
        self.resource_order
            .add_resource(name.to_string(), ResourceType::Object);
        let object = GmObject::new(name, parent);
        self.objects.push(object);
    }

    pub fn add_object_to_room(
        &mut self,
        room_name: &str,
        object_name: &str,
        x: f32,
        y: f32,
    ) -> std::io::Result<()> {
        // Find the room by name
        if let Some(room) = self.rooms.iter_mut().find(|r| r.name == room_name) {
            // Check if object exists in the project
            if !self.objects.iter().any(|o| o.name == object_name) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Object '{}' not found in project", object_name),
                ));
            }

            // Add the object to the room's instances
            let object_ref = ResourceRef {
                name: object_name.to_string(),
                path: format!("objects/{}/{}.yy", object_name, object_name),
            };

            room.add_instance(object_ref, x, y);
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Room '{}' not found", room_name),
            ))
        }
    }

    pub fn add_event_to_object(
        &mut self,
        project_path: &std::path::PathBuf,
        object_name: &str,
        event_type: EventType,
        event_subtype: EventSubType,
        code: Option<String>,
    ) -> std::io::Result<()> {
        // Find the object by name
        if let Some(object) = self.objects.iter_mut().find(|o| o.name == object_name) {
            // Add the event to the object's events
            object.add_event(event_type, event_subtype.clone());
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Object '{}' not found", object_name),
            ));
        }

        // Write code to file or create file
        let event_type_str = event_type.as_str();
        let event_num = event_subtype.value();
        let code_file_path = format!(
            "objects/{}/{}_{}.gml",
            object_name, event_type_str, event_num
        );
        let full_path = project_path.parent().unwrap().join(&code_file_path);
        std::fs::File::create(&full_path)?;
        if let Some(code) = code {
            std::fs::write(&full_path, code)?;
        }

        Ok(())
    }
}
