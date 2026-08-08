mod formatter;
mod options;
pub mod resource;
mod resource_order;
mod yyp;

use options::Options;
pub use resource::{Resource, ResourceKind, ResourceTrait};
pub use resource_order::ResourceOrder;
use yyp::GmProjectYyp;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceId {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct GmProject {
    pub path: PathBuf,
    pub resource_id: ResourceId,
    pub yyp: GmProjectYyp,
    pub resource_order: ResourceOrder,
    pub options: Options,
    pub resources: HashMap<String, Resource>,
}

impl GmProject {
    pub fn new(name: &str, path: &PathBuf) -> Self {
        let file_name = path
            .file_name()
            .expect("Failed to get file name")
            .to_string_lossy()
            .to_string();

        GmProject {
            path: path.clone(),
            resource_id: ResourceId {
                name: name.to_string(),
                path: file_name,
            },
            yyp: GmProjectYyp::new(name),
            resource_order: ResourceOrder::new(),
            options: Options::new(name),
            resources: HashMap::new(),
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let parent_dir = self.path.parent().expect("Failed to get parent directory");

        // Create the parent directory if it doesn't exist
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }

        // Save the project file
        self.yyp
            .save(&std::path::Path::new(&self.path))
            .expect("Failed to save project file");

        // Save the resource order file
        let resource_order_path =
            parent_dir.join(format!("{}.resource_order", self.resource_id.name.clone()));
        self.resource_order
            .save(&resource_order_path)
            .expect("Failed to save resource order");

        // Save the options files
        let options_path = parent_dir.join("options");
        self.options
            .save(&options_path)
            .expect("Failed to save options");

        // Create the datafiles directory if it doesn't exist
        let datafiles_path = parent_dir.join("datafiles");
        if !datafiles_path.exists() {
            std::fs::create_dir_all(&datafiles_path)?;
        }

        // self.resources is a hashmap of strings and resources
        // self.yyp.resources is a vec of ResourceId, which has a name and a path
        // We need to let resources = a map of self.yyp.resources, so we can know where the path is
        let resources: HashMap<String, (String, &Resource)> = self
            .yyp
            .resources
            .iter()
            .filter_map(|resource| {
                let resource_name = &resource.id.name;
                let resource_path = &resource.id.path;

                self.resources
                    .get(resource_name)
                    .map(|res| (resource_name.clone(), (resource_path.clone(), res)))
            })
            .collect();

        for (_, (resource_path, resource)) in resources {
            let full_resource_path = parent_dir.join(&resource_path);
            resource
                .save(&full_resource_path)
                .expect("Failed to save resource");
        }

        Ok(())
    }

    pub fn load(project_file_path: &std::path::Path) -> std::io::Result<Self> {
        let yyp = GmProjectYyp::load(&project_file_path).expect("Failed to load project file");

        let file_name = project_file_path
            .file_name()
            .expect("Failed to get file name")
            .to_string_lossy()
            .to_string();
        let resource_id = ResourceId {
            name: yyp.name.clone(),
            path: file_name,
        };

        let project_path = project_file_path
            .parent()
            .expect("Failed to get project directory");

        let resource_order_path = project_path.join(format!("{}.resource_order", resource_id.name));
        let resource_order =
            ResourceOrder::load(&resource_order_path).expect("Failed to load resource order");

        let options_path = project_path.join("options");
        let options = Options::load(&options_path).expect("Failed to load options");

        let mut resources = HashMap::new();
        for resource in &yyp.resources {
            let resource_path = project_path.join(&resource.id.path);
            let loaded_resource = Resource::load(&resource_path).expect("Failed to load resource");
            resources.insert(resource.id.name.clone(), loaded_resource);
        }

        Ok(GmProject {
            path: project_file_path.to_path_buf(),
            resource_id,
            yyp,
            resource_order,
            options,
            resources,
        })
    }

    pub fn get_id_from_resource_name(&self, name: &str) -> Option<&ResourceId> {
        self.yyp
            .resources
            .iter()
            .find(|resource| resource.id.name == name)
            .map(|resource| &resource.id)
    }

    pub fn add_resource(
        &mut self,
        name: Option<String>,
        resource_kind: ResourceKind,
    ) -> std::io::Result<()> {
        let name = name.unwrap_or_else(|| {
            let mut counter = 1u32;
            let mut name = format!("{}{}", resource_kind, counter);

            while self.resource_exists(&name) {
                counter += 1;
                name = format!("{}{}", resource_kind, counter);
            }

            name
        });

        let resource = Resource::new(&name, resource_kind.clone(), self.resource_id.clone());
        let resource_name = resource.name().to_string();
        let resource_path = resource.default_path();

        if self.resource_exists(&name) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("Resource with name '{}' already exists", name),
            ));
        }

        // Add the resource to the resources hashmap
        self.resources.insert(resource_name.clone(), resource);

        // Add the resource to the yyp file
        self.yyp
            .add_resource(&resource_kind, resource_name.clone(), resource_path.clone());

        self.resource_order
            .add_resource(&resource_kind, resource_name.clone(), resource_path);

        Ok(())
    }

    pub fn add_event_to_object(
        &mut self,
        object_name: &str,
        event_type: String,
        event_code: Option<String>,
    ) -> std::io::Result<()> {
        if let Some(resource) = self.resources.get_mut(object_name) {
            if let ResourceKind::Object = resource.kind() {
                let object = resource.as_object_mut().expect("Resource is not an object");
                object.add_event(event_type, event_code);
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Resource '{}' is not an object", object_name),
                ))
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Object '{}' not found", object_name),
            ))
        }
    }

    pub fn add_instance_to_room(
        &mut self,
        room_name: &str,
        object_name: &str,
        x: f32,
        y: f32,
    ) -> std::io::Result<()> {
        let object_id = self
            .get_id_from_resource_name(object_name)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Resource ID for '{}' not found", object_name),
                )
            })?
            .clone();

        let room_id = self
            .get_id_from_resource_name(room_name)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Resource ID for '{}' not found", room_name),
                )
            })?
            .clone();

        let room = self
            .resources
            .get_mut(room_name)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Room '{}' not found", room_name),
                )
            })?
            .as_room_mut()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Resource '{}' is not a room", room_name),
                )
            })?;

        room.add_instance(room_id, object_id, x, y);

        Ok(())
    }

    pub fn resource_exists(&self, name: &str) -> bool {
        self.resources.contains_key(name)
    }

    pub fn get_resource_path(&self, name: &str) -> Option<String> {
        self.yyp
            .resources
            .iter()
            .find(|resource| resource.id.name == name)
            .map(|resource| resource.id.path.clone())
    }
}
