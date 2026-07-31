use std::collections::HashMap;

use crate::{
    data_win::{ChunkBuilder, StringPool},
    project::GmObject,
};

#[derive(Debug, Clone)]
pub struct PhysicsVertex {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct CompiledObject {
    pub name: String,

    pub sprite: i32,
    pub parent: i32,
    pub mask: i32,

    pub visible: bool,
    pub managed: bool,
    pub solid: bool,
    pub depth: i32,
    pub persistent: bool,

    pub physics_angular_damping: f32,
    pub physics_density: f32,
    pub physics_friction: f32,
    pub physics_group: i32,
    pub physics_kinematic: bool,
    pub physics_linear_damping: f32,
    pub physics_object: bool,
    pub physics_restitution: f32,
    pub physics_sensor: bool,
    pub physics_shape: i32,
    pub physics_start_awake: bool,

    pub physics_vertices: Vec<PhysicsVertex>,

    pub event_lists: [Vec<CompiledEvent>; 15],
}

#[derive(Debug, Clone)]
pub struct CompiledEvent {
    pub event_type: i32,
    pub event_num: i32,
    pub collision_object: i32,
    pub code_index: u32,
}

impl CompiledObject {
    pub fn from_gmobject(
        object: &GmObject,
        sprite_lookup: &HashMap<String, u32>,
        object_lookup: &HashMap<String, u32>,
        code_lookup: &HashMap<(String, i32, i32), u32>,
    ) -> Self {
        let events: Vec<CompiledEvent> = object
            .event_list
            .iter()
            .map(|event| {
                let collision_object = event
                    .collision_object_id
                    .as_ref()
                    .and_then(|r| object_lookup.get(&r.name))
                    .map(|v| *v as i32)
                    .unwrap_or(-1);

                let code_index = *code_lookup
                    .get(&(object.name.clone(), event.event_type, event.event_num))
                    .expect("missing CODE entry");

                CompiledEvent {
                    event_type: event.event_type,
                    event_num: event.event_num,
                    collision_object,
                    code_index,
                }
            })
            .collect();

        let physics_vertices = object
            .physics_shape_points
            .chunks_exact(2)
            .map(|chunk| PhysicsVertex {
                x: chunk[0].as_f64().unwrap_or(0.0) as f32,
                y: chunk[1].as_f64().unwrap_or(0.0) as f32,
            })
            .collect();

        Self {
            name: object.name.clone(),

            sprite: object
                .sprite_id
                .as_ref()
                .and_then(|r| sprite_lookup.get(&r.name))
                .map(|v| *v as i32)
                .unwrap_or(-1),

            parent: object
                .parent_object_id
                .as_ref()
                .and_then(|r| object_lookup.get(&r.name))
                .map(|v| *v as i32)
                .unwrap_or(-1),

            mask: object
                .sprite_mask_id
                .as_ref()
                .and_then(|r| sprite_lookup.get(&r.name))
                .map(|v| *v as i32)
                .unwrap_or(-1),

            visible: object.visible,
            managed: object.managed,
            solid: object.solid,
            depth: 0, // TODO: Object YY files don't contain depth; GameMaker writes 0.
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

            physics_vertices,

            event_lists: {
                let mut lists: [Vec<CompiledEvent>; 15] = Default::default();
                for event in events {
                    if (event.event_type as usize) < lists.len() {
                        lists[event.event_type as usize].push(event);
                    } else {
                        eprintln!(
                            "Warning: Event type {} is out of bounds for object {}",
                            event.event_type, object.name
                        );
                    }
                }
                lists
            },
        }
    }
}

pub fn serialize_objects(objects: &[CompiledObject], pool: &mut StringPool) -> ChunkBuilder {
    const EVENT_TYPE_COUNT: usize = 15;

    let mut c = ChunkBuilder::new("OBJT");

    c.u32(objects.len() as u32);

    let object_ptrs: Vec<_> = (0..objects.len())
        .map(|_| c.local_ref_placeholder())
        .collect();

    for (object, ptr) in objects.iter().zip(object_ptrs) {
        let object_start = c.pos();
        c.local_ref_set(ptr, object_start);

        c.str_ref(pool, &object.name);

        c.i32(object.sprite);
        c.bool32(object.visible);
        c.bool32(true); // managed
        c.bool32(object.solid);
        c.i32(0); // depth
        c.bool32(object.persistent);
        c.i32(object.parent);
        c.i32(object.mask);

        c.bool32(object.physics_object);
        c.bool32(object.physics_sensor);

        c.u32(object.physics_shape as u32);

        c.f32(object.physics_density);
        c.f32(object.physics_restitution);

        c.u32(object.physics_group as u32);

        c.f32(object.physics_linear_damping);
        c.f32(object.physics_angular_damping);

        c.i32(object.physics_vertices.len() as i32);

        c.f32(object.physics_friction);
        c.bool32(object.physics_start_awake);
        c.bool32(object.physics_kinematic);

        for v in &object.physics_vertices {
            c.f32(v.x);
            c.f32(v.y);
        }

        //
        // outer pointer table
        //

        c.u32(EVENT_TYPE_COUNT as u32);

        let outer_ptrs: Vec<_> = (0..EVENT_TYPE_COUNT)
            .map(|_| c.local_ref_placeholder())
            .collect();

        for (event_type, outer_ptr) in outer_ptrs.iter().enumerate() {
            let events = &object.event_lists[event_type];

            let inner_start = c.pos();
            c.local_ref_set(*outer_ptr, inner_start);

            //
            // inner pointer table
            //

            c.u32(events.len() as u32);

            let inner_ptrs: Vec<_> = (0..events.len())
                .map(|_| c.local_ref_placeholder())
                .collect();

            //
            // actual events
            //

            for (event, inner_ptr) in events.iter().zip(inner_ptrs) {
                let event_start = c.pos();
                c.local_ref_set(inner_ptr, event_start);

                c.u32(event.event_num as u32);

                //
                // actions
                //

                // serialize_event_actions(&mut c, &event.actions);
            }
        }
    }

    c
}
