mod compiler;
mod data_win;
mod project;

use clap::{Parser as ClapParser, Subcommand};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(ClapParser)]
#[command(name = "gmlc")]
#[command(about = "GameMaker Language compiler")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        input: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    Create {
        project_name: String,
        folder_path: Option<PathBuf>,
    },
    AddResource {
        project_path: PathBuf,
        resource_type: String,
        resource_name: String,
    },
    AddEvent {
        project_path: PathBuf,
        object_name: String,
        event_type: String,
        event_subtype: Option<String>,
        code: Option<String>,
    },
    AddObjectToRoom {
        project_path: PathBuf,
        room_name: String,
        object_name: String,
        x: Option<f32>,
        y: Option<f32>,
    },
}

fn main() {
    let args: Args = ClapParser::parse();

    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    match args.command {
        Commands::Compile {
            input,
            output: output_path,
        } => {
            let project_file = PathBuf::from(&input);
            let project = match project::GmProject::load(&project_file) {
                Ok(proj) => proj,
                Err(e) => {
                    eprintln!("Failed to load project: {}", e);
                    return;
                }
            };

            let output = data_win::build_data_win_from_gmproject(project);
            // let input_content = std::fs::read_to_string(&input).expect("Failed to read input file");
            // let program = create_program_from_gml(&input_content);

            // print_disassembly(&program.bytecode);

            if let Some(output_path) = output_path {
                std::fs::write(&output_path, output).expect("Failed to write output file");
                println!("Output written to {}", output_path);
            } else {
                println!("No output path specified. Output not written.");
            }
        }
        Commands::Create {
            project_name,
            folder_path,
        } => {
            let project_path = folder_path.unwrap_or_else(|| {
                let mut path = std::env::current_dir().expect("Failed to get current directory");
                path.push(&project_name);
                path
            });
            let project_file_path = project_path.join(format!("{}.yyp", project_name));

            let project = project::GmProject::new(&project_name);
            if let Err(e) = project.save(&project_file_path) {
                eprintln!("Failed to initialize project folder: {}", e);
            } else {
                println!(
                    "Project '{}' initialized at '{}'",
                    project_name,
                    project_path.display()
                );
            }
        }
        Commands::AddResource {
            project_path,
            resource_type,
            resource_name,
        } => {
            let mut project = match project::GmProject::load(&project_path) {
                Ok(proj) => proj,
                Err(e) => {
                    eprintln!("Failed to load project: {}", e);
                    return;
                }
            };

            let resource_type_enum = match resource_type.as_str() {
                "object" => project::ResourceType::Object,
                "room" => project::ResourceType::Room,
                _ => {
                    eprintln!("Invalid resource type: {}", resource_type);
                    return;
                }
            };

            project.add_resource(resource_type_enum, &resource_name);
            project.save(&project_path).expect("Failed to save project");
        }
        Commands::AddObjectToRoom {
            project_path,
            room_name,
            object_name,
            x,
            y,
        } => {
            let mut project = match project::GmProject::load(&project_path) {
                Ok(proj) => proj,
                Err(e) => {
                    eprintln!("Failed to load project: {}", e);
                    return;
                }
            };

            let x = x.unwrap_or(0.0);
            let y = y.unwrap_or(0.0);

            if let Err(e) = project.add_object_to_room(&room_name, &object_name, x, y) {
                eprintln!("Failed to add object to room: {}", e);
                return;
            }

            project.save(&project_path).expect("Failed to save project");
        }
        Commands::AddEvent {
            project_path,
            object_name,
            event_type,
            event_subtype,
            code,
        } => {
            let mut project = match project::GmProject::load(&project_path) {
                Ok(proj) => proj,
                Err(e) => {
                    eprintln!("Failed to load project: {}", e);
                    return;
                }
            };

            let event_type = match project::EventType::from_str(&event_type) {
                Ok(et) => et,
                Err(_) => {
                    eprintln!("Invalid event type: {}", event_type);
                    return;
                }
            };

            let event_subtype_i32 = event_subtype.unwrap().parse::<i32>().ok();
            let event_subtype =
                project::EventSubType::from_i32(event_type, event_subtype_i32.unwrap_or(0));

            if let Err(e) = project.add_event_to_object(
                &project_path,
                &object_name,
                event_type,
                event_subtype,
                code,
            ) {
                eprintln!("Failed to add event to object: {}", e);
                return;
            }

            project.save(&project_path).expect("Failed to save project");
        }
    }
}
