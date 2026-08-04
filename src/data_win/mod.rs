mod chunk;
mod compile;
mod compiler;
mod finalize;
mod instance_type;
mod layout;
mod model;
mod sections;
mod string_pool;

use std::collections::HashMap;
use std::path::Path;

use crate::data_win::chunk::ChunkBuilder;
use crate::data_win::compile::{compile_object, compile_room};
use crate::data_win::finalize::build_form;
use crate::data_win::layout::WadLayout;
use crate::data_win::model::{
    CompiledCodeEntry, CompiledFunction, CompiledObject, CompiledRoom, CompiledVariable, Gen8, Optn,
};
use crate::data_win::string_pool::StringPool;
use crate::project::GmProject;
use compiler::resolver::{Resolver, Variable as ResolvedVariable};
use instance_type::InstanceType;

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
            sections::vari::build(
                &self.variables,
                &mut pool,
                &layout,
                self.max_local_var_count,
            ),
            sections::func::build(&self.functions, &mut pool, &layout, self.code_locals_count),
            sections::strg::build_placeholder(),
            sections::txtr::build(),
            sections::audo::build(),
        ];

        build_form(&mut chunks, &pool)
    }

    pub fn from_project(project: GmProject) -> Self {
        let mut data_win = DataWin::default();
        let project_root = project
            .path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| Path::new(".").to_path_buf());

        let mut object_id_by_name: HashMap<String, u32> = HashMap::new();
        let mut next_object_id = 0;
        for (name, res) in &project.resources {
            use crate::project::resource::Resource;
            if let Resource::Object(_) = res {
                object_id_by_name.insert(name.clone(), next_object_id);
                next_object_id += 1;
            }
        }

        let mut resolver = Resolver::new();
        let mut code_lookup: HashMap<(String, i32, i32), u32> = HashMap::new();
        let mut code_entries: Vec<CompiledCodeEntry> = Vec::new();

        let resources = project.resources.clone();
        for (name, res) in resources {
            use crate::project::resource::Resource;
            let resource_path = project.get_resource_path(&name).unwrap_or_default();
            let object_path = project_root.join(resource_path);

            match res {
                Resource::Room(room) => {
                    let compiled_room = compile_room(&room, &object_id_by_name);
                    data_win.rooms.push(compiled_room);
                }
                Resource::Object(object) => {
                    let compiled_object = compile_object(
                        &object,
                        &object_path,
                        &object_id_by_name,
                        &mut resolver,
                        &mut code_lookup,
                        &mut code_entries,
                    );
                    data_win.objects.push(compiled_object);
                }
                Resource::Script(script) => {}
            }
        }

        data_win.code = code_entries;
        let mut resolved_functions: Vec<_> = resolver.functions.values().collect();
        resolved_functions.sort_unstable_by_key(|function| function.var_ref);
        data_win.functions = resolved_functions
            .into_iter()
            .map(|function| CompiledFunction {
                name: function.name.clone(),
                occurrences: 0,
                first_address: -1,
            })
            .collect();
        data_win
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let data = self.build();
        std::fs::create_dir_all(path.parent().unwrap_or_else(|| Path::new(".")))?;
        std::fs::write(path, data)?;
        Ok(())
    }
}

fn strip_scope_prefix(name: &str) -> String {
    name.strip_prefix("global.")
        .or_else(|| name.strip_prefix("self."))
        .or_else(|| name.strip_prefix("other."))
        .or_else(|| name.strip_prefix("local."))
        .or_else(|| name.strip_prefix("static."))
        .or_else(|| name.strip_prefix("builtin."))
        .unwrap_or(name)
        .to_string()
}

fn resolved_variable_to_compiled(variable: &ResolvedVariable) -> CompiledVariable {
    let mut compiled =
        CompiledVariable::with_name(strip_scope_prefix(&variable.name), variable.var_ref as i32);

    // Set the VARI instance_type so the runtime knows which scope owns this variable.
    if variable.name.starts_with("global.") {
        compiled.instance_type = InstanceType::Global as i32;
    } else if variable.name.starts_with("self.") {
        compiled.instance_type = InstanceType::Self_ as i32;
    } else if variable.name.starts_with("other.") {
        compiled.instance_type = InstanceType::Other as i32;
    } else if variable.name.starts_with("local.") {
        compiled.instance_type = InstanceType::Local as i32;
    } else if variable.name.starts_with("static.") {
        compiled.instance_type = InstanceType::Static as i32;
    } else if variable.name.starts_with("builtin.") {
        // BC17: argument variables use instanceType -6 (INSTANCE_BUILTIN).
        // The runtime resolves the builtin var ID by name.
        compiled.instance_type = InstanceType::Builtin as i32;
    }

    compiled
}
