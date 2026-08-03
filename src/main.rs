// mod data_win;
mod project;
// pub mod types;

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use project::ResourceKind;

#[derive(Parser, Debug)]
#[command(name = "gmc")]
#[command(author, version, about)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub project: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new GameMaker project
    New(NewArgs),

    /// Build a project
    Build,

    /// Clean build output
    Clean,

    /// Add resources
    Add(AddArgs),

    /// Object operations
    Object(ObjectArgs),

    /// Room operations
    Room(RoomArgs),
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Project name
    pub name: String,

    /// Directory to create the project in
    #[arg(short, long, default_value = ".")]
    pub output: PathBuf,

    /// Project template
    #[arg(long, value_enum, default_value = "empty")]
    pub template: Template,

    /// Runtime version
    #[arg(long)]
    pub runtime: Option<String>,

    /// IDE version
    #[arg(long)]
    pub ide_version: Option<String>,

    /// Package identifier
    #[arg(long)]
    pub package: Option<String>,

    /// Display name
    #[arg(long)]
    pub display_name: Option<String>,

    /// Author
    #[arg(long)]
    pub author: Option<String>,

    /// Company
    #[arg(long)]
    pub company: Option<String>,

    /// Initial game version
    #[arg(long, default_value = "1.0.0")]
    pub version: String,

    /// Initialize git
    #[arg(long)]
    pub git: bool,

    /// Create README.md
    #[arg(long)]
    pub readme: bool,

    /// Create LICENSE
    #[arg(long)]
    pub license: Option<String>,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[command(subcommand)]
    pub resource: AddResourceCommand,
}

#[derive(Subcommand, Debug)]
pub enum AddResourceCommand {
    Object { name: String },
    Sprite { name: String },
    Script { name: String },
    Room { name: String },
    Font { name: String },
    Sound { name: String },
    Shader { name: String },
    Path { name: String },
    Sequence { name: String },
}

#[derive(Args, Debug)]
pub struct ObjectArgs {
    #[command(subcommand)]
    pub command: ObjectCommand,
}

#[derive(Subcommand, Debug)]
pub enum ObjectCommand {
    #[command(name = "add")]
    Add {
        #[command(subcommand)]
        command: AddObjectCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum AddObjectCommand {
    #[command(name = "event")]
    Event(AddEventArgs),
}

#[derive(Args, Debug)]
pub struct AddEventArgs {
    pub object: String,

    pub event: String,

    pub subtype: Option<String>,
}

#[derive(Args, Debug)]
pub struct RoomArgs {
    #[command(subcommand)]
    pub command: RoomCommand,
}

#[derive(Subcommand, Debug)]
pub enum RoomCommand {
    #[command(name = "add")]
    Add {
        #[command(subcommand)]
        command: AddRoomCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum AddRoomCommand {
    #[command(name = "instance")]
    Instance(AddInstanceArgs),
}

#[derive(Args, Debug)]
pub struct AddInstanceArgs {
    pub room: String,
    pub object: String,
    pub x: f32,
    pub y: f32,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Template {
    Empty,
    Platformer,
    Topdown,
}

fn main() {
    let cli = Cli::parse();
    let project_path_buf = cli.project.clone().unwrap_or_else(|| PathBuf::from("."));
    let project_path = project_path_buf.as_path();

    match &cli.command {
        Command::New(args) => {
            let output = args.output.join(&args.name);
            println!("Creating new project {} at {}", args.name, output.display());
            let project_file_path = output.join(format!("{}.yyp", args.name));

            let project = project::GmProject::new(&args.name, &project_file_path);
            if let Err(e) = project.save() {
                eprintln!("Error saving project: {}", e);
            } else {
                println!("Project created successfully!");
            }
        }
        Command::Add(args) => {
            let mut project = match project::GmProject::load(project_path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error loading project: {}", e);
                    return;
                }
            };

            let (resource_kind, name) = match &args.resource {
                AddResourceCommand::Room { name } => (ResourceKind::Room, name.clone()),
                AddResourceCommand::Object { name } => (ResourceKind::Object, name.clone()),
                _ => {
                    eprintln!("Unsupported resource type for adding");
                    return;
                }
            };

            if !project.resource_exists(&name) {
                project
                    .add_resource(&name, resource_kind)
                    .expect("Failed to add resource");
            }

            if let Err(e) = project.save() {
                eprintln!("Error saving project: {}", e);
            } else {
                println!("Resource added successfully");
            }
        }
        Command::Object(args) => match &args.command {
            ObjectCommand::Add {
                command: AddObjectCommand::Event(event_args),
            } => {
                let mut project = match project::GmProject::load(project_path) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Error loading project: {}", e);
                        return;
                    }
                };

                if let Err(e) = project.add_event_to_object(
                    &event_args.object,
                    event_args.event.clone(),
                    event_args.subtype.clone(),
                ) {
                    eprintln!("Error adding event to object: {}", e);
                } else {
                    if let Err(e) = project.save() {
                        eprintln!("Error saving project: {}", e);
                    } else {
                        println!("Event added to object successfully");
                    }
                }
            }
        },
        Command::Room(args) => match &args.command {
            RoomCommand::Add {
                command: AddRoomCommand::Instance(instance_args),
            } => {
                let mut project = match project::GmProject::load(project_path) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Error loading project: {}", e);
                        return;
                    }
                };

                if let Err(e) = project.add_instance_to_room(
                    &instance_args.room,
                    &instance_args.object,
                    instance_args.x,
                    instance_args.y,
                ) {
                    eprintln!("Error adding instance to room: {}", e);
                } else {
                    if let Err(e) = project.save() {
                        eprintln!("Error saving project: {}", e);
                    } else {
                        println!("Instance added to room successfully");
                    }
                }
            }
        },
        Command::Build => {
            let project = match project::GmProject::load(project_path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error loading project: {}", e);
                    return;
                }
            };

            // let output = data_win::build_project(&project);
        }
        Command::Clean => {
            println!("Cleaning build output");
            // Implement clean logic here
        }
    }
}
