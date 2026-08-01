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
    CompiledCodeEntry, CompiledCodeOwner, CompiledObject, CompiledRoom, Gen8, Optn,
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
    pub variables: Vec<String>,
    pub functions: Vec<String>,
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
            sections::vari::build(&self.variables, &mut pool, &layout),
            sections::func::build(&self.functions, &mut pool, &layout),
            sections::strg::build_placeholder(),
            sections::txtr::build(),
            sections::audo::build(),
        ];

        build_form(&mut chunks, &pool)
    }
}

pub fn build_data_win(code_name: &str, program: Program) -> Vec<u8> {
    DataWin {
        rooms: vec![CompiledRoom {
            creation_code_id: 0,
            ..CompiledRoom::default()
        }],
        code: vec![CompiledCodeEntry {
            owner: CompiledCodeOwner::Script {
                name: code_name.to_string(),
            },
            name: code_name.to_string(),
            bytecode: program.bytecode.data,
        }],
        variables: program.variables.into_iter().map(|variable| variable.name).collect(),
        functions: program.functions.into_iter().map(|function| function.name).collect(),
        ..DataWin::default()
    }
    .build()
}

pub fn build_data_win_multi(
    code_entries: &[CompiledCodeEntry],
    variables: &[String],
    functions: &[String],
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

    let mut functions = Vec::new();
    let mut variables = Vec::new();
    let code = project
        .code
        .iter()
        .map(|entry| compile_code_entry(entry, &object_id_by_name, &mut functions, &mut variables))
        .collect::<Vec<_>>();

    dedupe_in_place(&mut functions);
    dedupe_in_place(&mut variables);

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
