mod compiler;
mod data_win;
mod project;

use clap::{ArgAction, Args as ClapArgs, Parser as ClapParser};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(ClapParser)]
#[command(name = "gmlc")]
#[command(about = "GameMaker Language compiler")]
struct Args {
    #[command(flatten)]
    pipeline: PipelineArgs,
}

#[derive(ClapArgs, Debug, Default)]
struct PipelineArgs {
    /// Create an in-memory project for chained operations.
    #[arg(long)]
    create: Option<String>,

    /// Add a resource to the in-memory project. Repeatable.
    /// Example: --add-resource room Room1 --add-resource object Object1
    #[arg(
        long = "add-resource",
        value_names = ["TYPE", "NAME"],
        num_args = 2,
        action = ArgAction::Append
    )]
    add_resources: Vec<String>,

    /// Add an object instance to a room in the in-memory project. Repeatable.
    /// Example: --add-object-to-room Room1 Object1 100 120
    #[arg(
        long = "add-object-to-room",
        value_names = ["ROOM", "OBJECT", "X", "Y"],
        num_args = 4,
        action = ArgAction::Append
    )]
    add_objects_to_room: Vec<String>,

    /// Add an event to an object in the in-memory project. Repeatable.
    /// Example (inline): --add-event Object1 create 0 "x += 1;"
    /// Example (file): --add-event Object1 create 0 ./scripts/init.gml
    #[arg(
        long = "add-event",
        value_names = ["OBJECT", "EVENT_TYPE", "EVENT_SUBTYPE", "CODE"],
        num_args = 4,
        action = ArgAction::Append
    )]
    add_events: Vec<String>,

    /// Compile the in-memory project directly to a data.win file.
    #[arg(long = "compile", value_name = "OUTPUT_DATA_WIN")]
    compile_output: Option<PathBuf>,

    /// Optionally persist the in-memory project to disk.
    /// Pass either a .yyp file path or a directory.
    #[arg(long = "save-project", value_name = "PROJECT_PATH")]
    save_project: Option<PathBuf>,
}

fn main() {
    let args: Args = ClapParser::parse();

    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    if !is_pipeline_mode(&args.pipeline) {
        eprintln!(
            "No pipeline arguments provided. Use flags like --create, --add-resource, --add-event, --add-object-to-room, and --compile."
        );
        return;
    }

    run_pipeline(args.pipeline);
}

fn is_pipeline_mode(pipeline: &PipelineArgs) -> bool {
    pipeline.create.is_some()
        || !pipeline.add_resources.is_empty()
        || !pipeline.add_events.is_empty()
        || !pipeline.add_objects_to_room.is_empty()
        || pipeline.compile_output.is_some()
        || pipeline.save_project.is_some()
}

fn run_pipeline(pipeline: PipelineArgs) {
    let wants_compile_output = pipeline.compile_output.is_some();
    let wants_save_project = pipeline.save_project.is_some();

    let Some(project_name) = pipeline.create else {
        eprintln!("Pipeline mode requires --create <PROJECT_NAME>");
        return;
    };

    let mut project = project::GmProject::new(&project_name);

    for chunk in pipeline.add_resources.chunks_exact(2) {
        let resource_type = chunk[0].to_lowercase();
        let resource_name = &chunk[1];

        let resource_type_enum = match resource_type.as_str() {
            "object" => project::ResourceType::Object,
            "room" => project::ResourceType::Room,
            _ => {
                eprintln!("Invalid resource type: {}", chunk[0]);
                return;
            }
        };

        project.add_resource(resource_type_enum, resource_name);
    }

    for chunk in pipeline.add_objects_to_room.chunks_exact(4) {
        let room_name = &chunk[0];
        let object_name = &chunk[1];

        let x = match chunk[2].parse::<f32>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid x coordinate: {}", chunk[2]);
                return;
            }
        };

        let y = match chunk[3].parse::<f32>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid y coordinate: {}", chunk[3]);
                return;
            }
        };

        if let Err(e) = project.add_object_to_room(room_name, object_name, x, y) {
            eprintln!("Failed to add object to room: {}", e);
            return;
        }
    }

    for chunk in pipeline.add_events.chunks_exact(4) {
        let object_name = &chunk[0];

        let event_type = match parse_pipeline_event_type(&chunk[1]) {
            Ok(event_type) => event_type,
            Err(_) => {
                eprintln!("Invalid event type: {}", chunk[1]);
                return;
            }
        };

        let event_subtype_i32 = match chunk[2].parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid event subtype: {}", chunk[2]);
                return;
            }
        };

        let event_subtype = project::EventSubType::from_i32(event_type, event_subtype_i32);

        let Some(object) = project
            .objects
            .iter_mut()
            .find(|object| object.name == *object_name)
        else {
            eprintln!("Object '{}' not found in project", object_name);
            return;
        };

        object.add_event(event_type, event_subtype.clone());

        let event_code = match resolve_event_code_arg(&chunk[3]) {
            Ok(code) => code,
            Err(error) => {
                eprintln!("{}", error);
                return;
            }
        };

        // Keep event code in-memory so pipeline compile can include it without writing project files.
        project.code.push(project::CodeEntry::new_object_event(
            object_name,
            event_type,
            event_subtype,
            &event_code,
        ));
    }

    if let Some(output_path) = pipeline.compile_output {
        let output = data_win::build_data_win_from_gmproject(project.clone());
        if let Err(e) = std::fs::write(&output_path, output) {
            eprintln!("Failed to write data.win output: {}", e);
            return;
        }
        println!("Output written to {}", output_path.display());
    }

    if let Some(save_path) = pipeline.save_project {
        let project_file_path = resolve_project_file_path(&project, &save_path);
        if let Err(e) = project.save(&project_file_path) {
            eprintln!("Failed to save project: {}", e);
            return;
        }
        println!("Project saved to {}", project_file_path.display());
    }

    if !wants_compile_output && !wants_save_project {
        println!("Pipeline completed in memory (no disk output requested).");
    }
}

fn resolve_event_code_arg(value: &str) -> Result<String, String> {
    if value.to_ascii_lowercase().ends_with(".gml") {
        std::fs::read_to_string(value)
            .map_err(|error| format!("Failed to read .gml event code file '{}': {}", value, error))
    } else {
        Ok(value.to_string())
    }
}

fn resolve_project_file_path(project: &project::GmProject, save_path: &Path) -> PathBuf {
    if save_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("yyp"))
        .unwrap_or(false)
    {
        return save_path.to_path_buf();
    }

    save_path.join(format!("{}.yyp", project.yyp.name))
}

fn parse_pipeline_event_type(value: &str) -> Result<project::EventType, ()> {
    project::EventType::from_str(value).or_else(|_| match value.to_ascii_lowercase().as_str() {
        "create" => Ok(project::EventType::Create),
        "destroy" => Ok(project::EventType::Destroy),
        "alarm" => Ok(project::EventType::Alarm),
        "step" => Ok(project::EventType::Step),
        "collision" => Ok(project::EventType::Collision),
        "keyboard" => Ok(project::EventType::Keyboard),
        "mouse" => Ok(project::EventType::Mouse),
        "other" => Ok(project::EventType::Other),
        "draw" => Ok(project::EventType::Draw),
        "keypress" => Ok(project::EventType::KeyPress),
        "keyrelease" => Ok(project::EventType::KeyRelease),
        "cleanup" => Ok(project::EventType::Cleanup),
        "precreate" => Ok(project::EventType::PreCreate),
        _ => Err(()),
    })
}
