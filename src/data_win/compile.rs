use std::collections::HashMap;
use std::path::Path;

use super::compiler::{
    ast::{FunctionParameter, Statement},
    compiler::{Compiler, StructConstructor},
    disassembler::print_disassembly,
    lexer::Lexer,
    parser::Parser,
    resolver::Resolver,
};
use crate::data_win::model::{
    CompiledCodeEntry, CompiledCodeOwner, CompiledEvent, CompiledInstance, CompiledObject,
    CompiledRoom, PhysicsVertex,
};
use crate::project::resource::{Object, Room};

fn collect_function_declarations(
    statements: &[Statement],
    declarations: &mut Vec<(String, Vec<FunctionParameter>, Vec<Statement>)>,
) {
    for statement in statements {
        match statement {
            Statement::FunctionDeclaration {
                name, params, body, ..
            } => {
                declarations.push((name.clone(), params.clone(), body.clone()));
            }
            Statement::If {
                then_branch,
                else_branch,
                ..
            } => {
                collect_function_declarations(then_branch, declarations);
                if let Some(else_branch) = else_branch {
                    collect_function_declarations(else_branch, declarations);
                }
            }
            Statement::While { body, .. } => {
                collect_function_declarations(body, declarations);
            }
            Statement::Repeat { body, .. } => {
                collect_function_declarations(body, declarations);
            }
            Statement::For { body, .. } => {
                collect_function_declarations(body, declarations);
            }
            Statement::DoUntil { body, .. } => {
                collect_function_declarations(body, declarations);
            }
            Statement::Switch { cases, default, .. } => {
                for (_, body) in cases {
                    collect_function_declarations(body, declarations);
                }
                if let Some(default_body) = default {
                    collect_function_declarations(default_body, declarations);
                }
            }
            _ => {}
        }
    }
}

pub fn compile_code_entry(
    owner: CompiledCodeOwner,
    name: String,
    code: String,
    resolver: &mut Resolver,
) -> Vec<CompiledCodeEntry> {
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let program_ast = parser.parse_program();
    log::debug!("Parsed AST: {:#?}", program_ast);

    let mut compiler = Compiler::with_struct_name_prefix(name.clone());
    compiler.compile_program(&program_ast);
    let struct_constructors = compiler.struct_constructors.clone();
    let program = resolver.resolve(compiler.instructions);

    print_disassembly(&program.bytecode);

    let mut compiled = CompiledCodeEntry::new(owner.clone(), name.clone(), program.bytecode.data);
    compiled.string_fixups = program.bytecode.string_fixups;

    let mut compiled_entries = vec![compiled];

    let mut function_declarations = Vec::new();
    collect_function_declarations(&program_ast, &mut function_declarations);
    for (function_name, params, body) in function_declarations {
        let script_name = format!("gml_Script_{}@{}", function_name, name);
        let mut script_compiler = Compiler::with_struct_name_prefix(name.clone());
        script_compiler.compile_function_body(&params, &body);
        let script_program = resolver.resolve(script_compiler.instructions);
        let mut script_entry = CompiledCodeEntry::new(
            CompiledCodeOwner::Script {
                name: script_name.clone(),
            },
            script_name.clone(),
            script_program.bytecode.data,
        );
        script_entry.string_fixups = script_program.bytecode.string_fixups;
        script_entry.arguments_count = params.len() as u16;
        compiled_entries.push(script_entry);
    }

    if matches!(owner, CompiledCodeOwner::ObjectEvent { .. }) {
        for constructor in struct_constructors {
            compiled_entries.extend(compile_struct_constructor_entries(&constructor, resolver));
        }
    }

    compiled_entries
}

fn compile_struct_constructor_entries(
    constructor: &StructConstructor,
    resolver: &mut Resolver,
) -> Vec<CompiledCodeEntry> {
    let statements = constructor
        .fields
        .iter()
        .map(|(key, value)| Statement::Assignment {
            name: format!("self.{}", key),
            value: value.clone(),
        })
        .collect::<Vec<_>>();

    let mut compiler = Compiler::with_struct_name_prefix(constructor.name.clone());
    compiler.compile_program(&statements);
    let nested_constructors = compiler.struct_constructors.clone();
    let program = resolver.resolve(compiler.instructions);

    let mut compiled = CompiledCodeEntry::new(
        CompiledCodeOwner::Script {
            name: constructor.name.clone(),
        },
        constructor.name.clone(),
        program.bytecode.data,
    );
    compiled.string_fixups = program.bytecode.string_fixups;

    let mut entries = vec![compiled];
    for nested in nested_constructors {
        entries.extend(compile_struct_constructor_entries(&nested, resolver));
    }

    entries
}

pub fn compile_room(room: &Room, object_id_by_name: &HashMap<String, u32>) -> CompiledRoom {
    use crate::project::resource::room::LayerTrait;

    let instances = room
        .layers
        .iter()
        .flat_map(|layer| layer.instances().into_iter().flatten())
        .enumerate()
        .map(|(index, instance)| {
            let object_name = instance
                .object
                .as_ref()
                .map(|resource| resource.name.clone())
                .unwrap_or_default();

            CompiledInstance {
                id: index as u32,
                object_id: object_id_by_name.get(&object_name).copied().unwrap_or(0),
                x: instance.x,
                y: instance.y,
                ..CompiledInstance::default()
            }
        })
        .collect::<Vec<_>>();

    CompiledRoom {
        name: room.name.clone(),
        width: room.room_settings.width,
        height: room.room_settings.height,
        creation_code_id: -1,
        flags: 0,
        world: room.physics_settings.physics_world,
        gravity_x: room.physics_settings.physics_world_gravity_x,
        gravity_y: room.physics_settings.physics_world_gravity_y,
        meters_per_pixel: room.physics_settings.physics_world_pix_to_metres,
        instances,
        ..CompiledRoom::default()
    }
}

pub fn compile_object(
    object: &Object,
    object_path: &Path,
    object_id_by_name: &HashMap<String, u32>,
    resolver: &mut Resolver,
    code_lookup: &mut HashMap<(String, i32, i32), u32>,
    code_entries: &mut Vec<CompiledCodeEntry>,
) -> CompiledObject {
    for event in &object.event_list {
        let key = (object.name.clone(), event.event_type, event.event_num);

        let code = event.get_code(object_path).unwrap_or_default();
        let entry_name = format!("{}_{}_{}", object.name, event.event_type, event.event_num);
        let owner = CompiledCodeOwner::ObjectEvent {
            object_id: object_id_by_name.get(&object.name).copied().unwrap_or(0),
            event_type: event.event_type,
            event_num: event.event_num,
        };

        let compiled = compile_code_entry(owner, entry_name, code, resolver);
        let code_index = code_entries.len() as u32;
        code_lookup.insert(key, code_index);
        code_entries.extend(compiled);
    }

    let mut event_lists: [Vec<CompiledEvent>; 15] = Default::default();
    for event in &object.event_list {
        let key = (object.name.clone(), event.event_type, event.event_num);
        let code_index = code_lookup
            .get(&key)
            .copied()
            .map(|index| index as i32)
            .unwrap_or(-1);

        let type_index = event.event_type as usize;
        if type_index < event_lists.len() {
            event_lists[type_index].push(CompiledEvent {
                event_num: event.event_num,
                actions: vec![crate::data_win::model::CompiledEventAction {
                    lib_id: 1,
                    id: 603,
                    kind: 7,
                    use_relative: false,
                    is_question: false,
                    use_apply_to: false,
                    exe_type: 2,
                    action_name: String::new(),
                    code_id: code_index,
                    argument_count: 0,
                    who: -1,
                    relative: false,
                    is_not: false,
                    unknown_always_zero: 0,
                }],
            });
        }
    }

    let physics_vertices = object
        .physics_shape_points
        .chunks_exact(2)
        .map(|pair| PhysicsVertex {
            x: pair[0].as_f64().unwrap_or(0.0) as f32,
            y: pair[1].as_f64().unwrap_or(0.0) as f32,
        })
        .collect::<Vec<_>>();

    CompiledObject {
        name: object.name.clone(),
        sprite: -1,
        parent: object
            .parent_object_id
            .as_ref()
            .and_then(|resource| object_id_by_name.get(&resource.name).copied())
            .map(|id| id as i32)
            .unwrap_or(-1),
        mask: -1,
        visible: object.visible,
        managed: object.managed,
        solid: object.solid,
        depth: 0,
        persistent: object.persistent,
        physics_angular_damping: object.physics_angular_damping,
        physics_density: object.physics_density,
        physics_friction: object.physics_friction,
        physics_group: object.physics_group,
        physics_kinematic: object.physics_kinematic,
        physics_linear_damping: object.physics_linear_damping,
        physics_object: object.physics_object,
        physics_restitution: object.physics_restitution,
        physics_sensor: object.physics_sensor,
        physics_shape: object.physics_shape,
        physics_start_awake: object.physics_start_awake,
        event_type_count: 15,
        physics_vertices,
        event_lists,
    }
}
