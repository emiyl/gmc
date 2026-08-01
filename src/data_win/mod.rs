mod chunk;
mod compile;
mod finalize;
mod layout;
mod model;
mod sections;
mod string_pool;

use std::collections::{HashMap, HashSet};

use crate::compiler::Program;
use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::compile::{compile_code_entry, compile_object, compile_room};
use crate::data_win::finalize::build_form;
use crate::data_win::layout::WadLayout;
use crate::data_win::model::{
    CompiledCodeEntry, CompiledCodeOwner, CompiledFunction, CompiledObject, CompiledRoom,
    CompiledVariable, Gen8, Optn,
};
use crate::data_win::string_pool::StringPool;
use crate::project::GmProject;

pub struct DataWin {
    pub wad_version: u8,
    pub gen8: Gen8,
    pub optn: Optn,
    pub rooms: Vec<CompiledRoom>,
    pub objects: Vec<CompiledObject>,
    pub code: Vec<CompiledCodeEntry>,
    pub variables: Vec<CompiledVariable>,
    pub functions: Vec<CompiledFunction>,
    pub max_local_var_count: u32,
    pub code_locals_count: u32,
}

impl Default for DataWin {
    fn default() -> Self {
        Self {
            wad_version: 17,
            gen8: Gen8::default(),
            optn: Optn::default(),
            rooms: Vec::new(),
            objects: Vec::new(),
            code: Vec::new(),
            variables: Vec::new(),
            functions: Vec::new(),
            max_local_var_count: 0,
            code_locals_count: 0,
        }
    }
}

impl DataWin {
    pub fn build(&self) -> Vec<u8> {
        let layout = WadLayout::for_version(self.wad_version);
        let mut pool = StringPool::new();

        let mut chunks: Vec<ChunkBuilder> = vec![
            sections::gen8::build(&self.gen8, &mut pool, &layout),
            sections::optn::build(&self.optn, &layout),
            sections::extn::build(),
            sections::sond::build(),
            sections::agrp::build(),
            sections::sprt::build(),
            sections::bgnd::build(),
            sections::path::build(),
            sections::scpt::build(),
            sections::glob::build(),
            sections::shdr::build(),
            sections::font::build(),
            sections::tmln::build(),
            sections::objt::build(&self.objects, &mut pool),
            sections::room::build(&self.rooms, &mut pool, &layout),
            sections::tpag::build(),
            sections::code::build(&self.code, &mut pool, &layout),
            sections::vari::build(&self.variables, &mut pool, &layout, self.max_local_var_count),
            sections::func::build(&self.functions, &mut pool, &layout, self.code_locals_count),
            sections::strg::build_placeholder(),
            sections::txtr::build(),
            sections::audo::build(),
        ];

        build_form(&mut chunks, &pool)
    }
}

pub fn build_data_win(code_name: &str, program: Program) -> Vec<u8> {
    let code_entry = CompiledCodeEntry::new(
        CompiledCodeOwner::Script {
            name: code_name.to_string(),
        },
        code_name.to_string(),
        program.bytecode.data,
    );

    DataWin {
        rooms: vec![CompiledRoom {
            creation_code_id: 0,
            ..CompiledRoom::default()
        }],
        code: vec![code_entry],
        variables: program
            .variables
            .into_iter()
            .enumerate()
            .map(|(index, variable)| CompiledVariable::with_name(variable.name, index as i32))
            .collect(),
        functions: program
            .functions
            .into_iter()
            .map(|function| CompiledFunction::with_name(function.name))
            .collect(),
        ..DataWin::default()
    }
    .build()
}

pub fn build_data_win_multi(
    code_entries: &[CompiledCodeEntry],
    variables: &[CompiledVariable],
    functions: &[CompiledFunction],
) -> Vec<u8> {
    DataWin {
        rooms: vec![CompiledRoom {
            creation_code_id: if code_entries.is_empty() { -1 } else { 0 },
            ..CompiledRoom::default()
        }],
        code: code_entries.to_vec(),
        variables: variables.to_vec(),
        functions: functions.to_vec(),
        ..DataWin::default()
    }
    .build()
}

pub fn build_data_win_from_gmproject(project: GmProject) -> Vec<u8> {
    let object_id_by_name = project
        .objects
        .iter()
        .enumerate()
        .map(|(index, object)| (object.name.clone(), index as u32))
        .collect::<HashMap<_, _>>();

    let object_name_by_id = object_id_by_name
        .iter()
        .map(|(name, id)| (*id, name.clone()))
        .collect::<HashMap<_, _>>();

    let rooms = project
        .rooms
        .into_iter()
        .map(|room| compile_room(room, &object_id_by_name))
        .collect::<Vec<_>>();

    let mut function_names = Vec::new();
    let mut variable_names = Vec::new();
    let code = project
        .code
        .iter()
        .map(|entry| {
            compile_code_entry(
                entry,
                &object_id_by_name,
                &mut function_names,
                &mut variable_names,
            )
        })
        .collect::<Vec<_>>();

    dedupe_in_place(&mut function_names);
    dedupe_in_place(&mut variable_names);

    let variables = variable_names
        .into_iter()
        .enumerate()
        .map(|(index, name)| CompiledVariable::with_name(name, index as i32))
        .collect::<Vec<_>>();
    let functions = function_names
        .into_iter()
        .map(CompiledFunction::with_name)
        .collect::<Vec<_>>();

    let code_lookup = code
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| match &entry.owner {
            CompiledCodeOwner::ObjectEvent {
                object_id,
                event_type,
                event_num,
            } => {
                let object_name = object_name_by_id.get(object_id)?;
                Some(((object_name.clone(), *event_type, *event_num), index as u32))
            }
            CompiledCodeOwner::Script { .. } => None,
        })
        .collect::<HashMap<_, _>>();

    let objects = project
        .objects
        .iter()
        .map(|object| compile_object(object, &object_id_by_name, &code_lookup))
        .collect::<Vec<_>>();

    let mut data_win = DataWin {
        rooms,
        objects,
        code,
        variables,
        functions,
        ..DataWin::default()
    };

    data_win.gen8.room_order = (0..data_win.rooms.len()).map(|index| index as i32).collect();

    data_win.build()
}

fn dedupe_in_place(values: &mut Vec<String>) {
    let mut seen = HashSet::new();
    values.retain(|value| seen.insert(value.clone()));
}
