mod data_win;
mod project;

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::{env, fs, path::PathBuf};

use project::ResourceKind;

#[derive(Parser, Debug)]
#[command(name = "gmc")]
#[command(author, version, about)]
pub struct Cli {
    /// Project file path or directory containing a .yyp file.
    #[arg(short, long, value_name = "PROJECT_PATH")]
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

    /// Run the project
    Run(RunArgs),

    /// Add resources
    Add(AddArgs),

    /// Object operations
    Object(ObjectArgs),

    /// Room operations
    Room(RoomArgs),
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// data.win runner binary path (default: butterscotch)
    #[arg(short, long, value_name = "RUNNER_PATH")]
    pub runner: Option<PathBuf>,
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

fn find_yyp_in_dir(dir: &std::path::Path) -> std::io::Result<PathBuf> {
    let mut candidates = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("yyp"))
                .unwrap_or(false)
        {
            candidates.push(path);
        }
    }

    match candidates.len() {
        0 => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No .yyp file found in directory {}", dir.display()),
        )),
        1 => Ok(candidates.remove(0)),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Multiple .yyp files found in {}. Please pass --project to specify one.",
                dir.display()
            ),
        )),
    }
}

fn resolve_project_file(project_arg: Option<&PathBuf>) -> std::io::Result<PathBuf> {
    let candidate = match project_arg {
        Some(path) => path.clone(),
        None => env::current_dir()?,
    };

    if candidate.is_dir() {
        find_yyp_in_dir(&candidate)
    } else if candidate
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("yyp"))
        .unwrap_or(false)
    {
        Ok(candidate)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Project path must be a .yyp file or directory containing a .yyp file: {}",
                candidate.display()
            ),
        ))
    }
}

fn main() {
    let cli = Cli::parse();
    let project_file_path = if matches!(&cli.command, Command::New(_)) {
        None
    } else {
        Some(
            resolve_project_file(cli.project.as_ref()).unwrap_or_else(|e| {
                eprintln!("Error resolving project: {}", e);
                std::process::exit(1);
            }),
        )
    };
    let project_path = project_file_path.as_ref().map(|p| p.as_path());

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
        Command::Build => {
            let project_path =
                project_path.expect("Project path must be resolved for non-New commands");
            let project = match project::GmProject::load(project_path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error loading project: {}", e);
                    return;
                }
            };

            let output = data_win::DataWin::from_project(project);
            let output_path = project_path
                .parent()
                .unwrap_or(project_path)
                .join("build")
                .join("data.win");

            if let Err(e) = output.save(&output_path) {
                eprintln!("Error saving build output: {}", e);
            } else {
                println!("Build output saved to {}", output_path.display());
            }
        }
        Command::Clean => {
            let project_path =
                project_path.expect("Project path must be resolved for non-New commands");
            let build_dir = project_path.parent().unwrap_or(project_path).join("build");

            if build_dir.exists() {
                if let Err(e) = fs::remove_dir_all(&build_dir) {
                    eprintln!("Error cleaning build output: {}", e);
                } else {
                    println!("Build output cleaned from {}", build_dir.display());
                }
            } else {
                println!("No build output to clean at {}", build_dir.display());
            }
        }
        Command::Run(args) => {
            let project_path =
                project_path.expect("Project path must be resolved for non-New commands");
            let project = match project::GmProject::load(project_path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error loading project: {}", e);
                    return;
                }
            };

            let build_dir = project_path.parent().unwrap_or(project_path).join("build");
            let data_win_path = build_dir.join("data.win");
            if !build_dir.exists() || !data_win_path.exists() {
                println!("Build output not found. Building project...");
                let output = data_win::DataWin::from_project(project);
                let output_path = build_dir.join("data.win");
                if let Err(e) = output.save(&output_path) {
                    eprintln!("Error saving build output: {}", e);
                    return;
                }
            }

            // Determine the runner path
            let runner_path = if let Some(runner) = &args.runner {
                runner.clone()
            } else {
                // see if binary "butterscotch" exists in PATH
                let butterscotch_path = which::which("butterscotch");
                match butterscotch_path {
                    Ok(path) => path,
                    Err(_) => {
                        eprintln!(
                            "butterscotch binary not found in PATH. Please install it to run the project."
                        );
                        return;
                    }
                }
            };

            let status = std::process::Command::new(runner_path)
                .arg(data_win_path)
                .status()
                .expect("Failed to execute runner");
            if !status.success() {
                eprintln!("Runner exited with status: {}", status);
            }
        }
        Command::Add(args) => {
            let project_path =
                project_path.expect("Project path must be resolved for non-New commands");
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
                AddResourceCommand::Script { name } => (ResourceKind::Script, name.clone()),
                AddResourceCommand::Sprite { name } => (ResourceKind::Sprite, name.clone()),
                _ => {
                    eprintln!("Unsupported resource type for adding");
                    return;
                }
            };

            if !project.resource_exists(&name) {
                project
                    .add_resource(&name, resource_kind)
                    .expect("Failed to add resource");
                if let Err(e) = project.save() {
                    eprintln!("Error saving project: {}", e);
                } else {
                    println!("Resource added successfully");
                }
            } else {
                println!("Error: Resource {} already exists.", name);
            }
        }
        Command::Object(args) => match &args.command {
            ObjectCommand::Add {
                command: AddObjectCommand::Event(event_args),
            } => {
                let project_path =
                    project_path.expect("Project path must be resolved for non-New commands");
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
                let project_path =
                    project_path.expect("Project path must be resolved for non-New commands");
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
    }
}
