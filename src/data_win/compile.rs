use std::collections::HashMap;

use crate::compiler::{
    compiler::Compiler, disassembler::print_disassembly, lexer::Lexer, parser::Parser,
    resolver::Resolver,
};
use crate::data_win::model::{
    CompiledCodeEntry, CompiledCodeOwner, CompiledEvent, CompiledInstance, CompiledObject,
    CompiledRoom, PhysicsVertex,
};
use crate::project::resources::gm_room_layer::LayerTrait;
use crate::project::{CodeEntry, CodeOwner, GmObject, GmRoom};

pub fn compile_code_entry(
    entry: &CodeEntry,
    object_id_by_name: &HashMap<String, u32>,
    functions: &mut Vec<String>,
    variables: &mut Vec<String>,
) -> CompiledCodeEntry {
    let lexer = Lexer::new(entry.code.clone());
    let mut parser = Parser::new(lexer);
    let program_ast = parser.parse_program();

    let mut compiler = Compiler::new();
    compiler.compile_program(&program_ast);

    let mut resolver = Resolver::new();
    let program = resolver.resolve(compiler.instructions);

    print_disassembly(&program.bytecode);

    let mut resolved_functions = resolver.functions.values().cloned().collect::<Vec<_>>();
    resolved_functions.sort_by_key(|function| function.var_ref);
    functions.extend(resolved_functions.into_iter().map(|function| function.name));

    let mut resolved_variables = resolver.variables.values().cloned().collect::<Vec<_>>();
    resolved_variables.sort_by_key(|variable| variable.var_ref);
    variables.extend(resolved_variables.into_iter().map(|variable| variable.name));

    let owner = match &entry.owner {
        CodeOwner::Script { name } => CompiledCodeOwner::Script { name: name.clone() },
        CodeOwner::ObjectEvent {
            object,
            event_type,
            event_num,
        } => CompiledCodeOwner::ObjectEvent {
            object_id: object_id_by_name.get(object).copied().unwrap_or(0),
            event_type: *event_type as i32,
            event_num: event_num.value(),
        },
    };

    CompiledCodeEntry::new(owner, entry.name.clone(), program.bytecode.data)
}

pub fn compile_room(room: GmRoom, object_id_by_name: &HashMap<String, u32>) -> CompiledRoom {
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
        name: room.name,
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
    object: &GmObject,
    object_id_by_name: &HashMap<String, u32>,
    code_lookup: &HashMap<(String, i32, i32), u32>,
) -> CompiledObject {
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
