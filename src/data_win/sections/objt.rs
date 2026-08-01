use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::model::{CompiledEventAction, CompiledObject};
use crate::data_win::string_pool::StringPool;

pub fn build(objects: &[CompiledObject], pool: &mut StringPool) -> ChunkBuilder {
    const MAX_EVENT_TYPE_SLOTS: usize = 15;

    let mut chunk = ChunkBuilder::new("OBJT");

    chunk.u32(objects.len() as u32);
    let object_ptrs = (0..objects.len())
        .map(|_| chunk.local_ref_placeholder())
        .collect::<Vec<_>>();

    for (object, object_ptr) in objects.iter().zip(object_ptrs.into_iter()) {
        let object_start = chunk.pos();
        chunk.local_ref_set(object_ptr, object_start);

        chunk.str_ref(pool, &object.name);
        chunk.i32(object.sprite);
        chunk.bool32(object.visible);
        chunk.bool32(object.managed);
        chunk.bool32(object.solid);
        chunk.i32(object.depth);
        chunk.bool32(object.persistent);
        chunk.i32(object.parent);
        chunk.i32(object.mask);

        chunk.bool32(object.physics_object);
        chunk.bool32(object.physics_sensor);
        chunk.u32(object.physics_shape as u32);
        chunk.f32(object.physics_density);
        chunk.f32(object.physics_restitution);
        chunk.u32(object.physics_group as u32);
        chunk.f32(object.physics_linear_damping);
        chunk.f32(object.physics_angular_damping);
        chunk.i32(object.physics_vertices.len() as i32);
        chunk.f32(object.physics_friction);
        chunk.bool32(object.physics_start_awake);
        chunk.bool32(object.physics_kinematic);

        for vertex in &object.physics_vertices {
            chunk.f32(vertex.x);
            chunk.f32(vertex.y);
        }

        let event_type_slots = object.event_type_count.min(MAX_EVENT_TYPE_SLOTS as u32) as usize;
        chunk.u32(event_type_slots as u32);
        let outer_ptrs = (0..event_type_slots)
            .map(|_| chunk.local_ref_placeholder())
            .collect::<Vec<_>>();

        for (event_type, outer_ptr) in outer_ptrs.iter().enumerate() {
            let events = &object.event_lists[event_type];

            let inner_start = chunk.pos();
            chunk.local_ref_set(*outer_ptr, inner_start);

            chunk.u32(events.len() as u32);
            let inner_ptrs = (0..events.len())
                .map(|_| chunk.local_ref_placeholder())
                .collect::<Vec<_>>();

            for (event, inner_ptr) in events.iter().zip(inner_ptrs.into_iter()) {
                let event_start = chunk.pos();
                chunk.local_ref_set(inner_ptr, event_start);

                chunk.u32(event.event_num as u32);
                serialize_event_actions(&mut chunk, &event.actions, pool);
            }
        }
    }

    chunk
}

fn serialize_event_actions(
    chunk: &mut ChunkBuilder,
    actions: &[CompiledEventAction],
    pool: &mut StringPool,
) {
    chunk.u32(actions.len() as u32);
    let action_ptrs = (0..actions.len())
        .map(|_| chunk.local_ref_placeholder())
        .collect::<Vec<_>>();

    for (action, action_ptr) in actions.iter().zip(action_ptrs.into_iter()) {
        let action_start = chunk.pos();
        chunk.local_ref_set(action_ptr, action_start);

        chunk.u32(action.lib_id);
        chunk.u32(action.id);
        chunk.u32(action.kind);
        chunk.bool32(action.use_relative);
        chunk.bool32(action.is_question);
        chunk.bool32(action.use_apply_to);
        chunk.u32(action.exe_type);
        chunk.str_ref(pool, &action.action_name);
        chunk.i32(action.code_id);
        chunk.u32(action.argument_count);
        chunk.i32(action.who);
        chunk.bool32(action.relative);
        chunk.bool32(action.is_not);
        chunk.u32(action.unknown_always_zero);
    }
}
