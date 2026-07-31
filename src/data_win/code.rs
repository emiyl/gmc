use std::collections::HashMap;

use super::WadLayout;
use crate::{
    compiler::compiler::Compiler,
    compiler::lexer::Lexer,
    compiler::parser::Parser,
    compiler::resolver::Resolver,
    data_win::{ChunkBuilder, Patch, StringPool},
    project::{CodeEntry, CodeOwner, EventSubType, EventType},
};

#[derive(Debug, Clone)]
pub enum CompiledCodeOwner {
    Script {
        name: String,
    },
    ObjectEvent {
        object: u32,
        event_type: EventType,
        event_num: EventSubType,
    },
}

#[derive(Debug, Clone)]
pub struct CompiledCodeEntry {
    pub owner: CompiledCodeOwner,
    pub name: String,
    pub bytecode: Vec<u8>,
}

impl CompiledCodeEntry {
    pub fn from_code_entry(
        entry: &CodeEntry,
        object_map: &HashMap<u32, String>,
        functions: &mut Vec<String>,
        variables: &mut Vec<String>,
    ) -> Self {
        let code_string = entry.code.clone();
        let lexer = Lexer::new(code_string);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse_program();

        let mut compiler = Compiler::new();
        compiler.compile_program(&program_ast);

        let mut resolver = Resolver::new();
        let program = resolver.resolve(compiler.instructions);

        functions.extend(resolver.functions.keys().cloned());
        variables.extend(resolver.variables.keys().cloned());

        let owner: CompiledCodeOwner = match &entry.owner {
            CodeOwner::Script { name } => CompiledCodeOwner::Script { name: name.clone() },
            CodeOwner::ObjectEvent {
                object,
                event_type,
                event_num,
            } => {
                let object: u32 = object_map
                    .iter()
                    .find(|(_, v)| *v == object)
                    .map(|(k, _)| *k)
                    .unwrap_or(0);

                CompiledCodeOwner::ObjectEvent {
                    object,
                    event_type: *event_type,
                    event_num: event_num.clone(),
                }
            }
        };

        Self {
            owner,
            name: entry.name.clone(),
            bytecode: program.bytecode.data,
        }
    }
}

/// CODE chunk: PointerList of compiled scripts (old / wadVersion <= 14 format).
/// Each entry is `name, length, <raw bytes>` - no locals/arguments header.
pub fn serialize_code(
    code: &[CompiledCodeEntry],
    pool: &mut StringPool,
    layout: &WadLayout,
) -> ChunkBuilder {
    let mut c = ChunkBuilder::new("CODE");

    c.u32(code.len() as u32);
    let ptr_positions: Vec<usize> = (0..code.len()).map(|_| c.local_ref_placeholder()).collect();

    if layout.old_code_format {
        for (entry, ptr_pos) in code.iter().zip(ptr_positions) {
            let entry_start = c.pos();
            c.local_ref_set(ptr_pos, entry_start);

            c.str_ref(pool, &entry.name);
            c.u32(entry.bytecode.len() as u32);
            c.bytes(&entry.bytecode);
        }
    } else {
        // New format: write every entry's fixed-size header first, then
        // the bytecode blobs, so bytecodeRelAddr can point forward.
        let mut rel_addr_field_positions = Vec::with_capacity(code.len());
        for (entry, ptr_pos) in code.iter().zip(&ptr_positions) {
            let entry_start = c.pos();
            c.patches.push(Patch::Local(*ptr_pos, entry_start));

            c.str_ref(pool, &entry.name);
            c.u32(entry.bytecode.len() as u32); // length
            c.u16(0); // localsCount
            c.u16(0); // argumentsCount
            let rel_addr_field_pos = c.pos();
            c.i32(0); // bytecodeRelAddr placeholder, fixed up below
            c.u32(0); // offset
            rel_addr_field_positions.push(rel_addr_field_pos);
        }

        for (entry, rel_addr_field_pos) in code.iter().zip(rel_addr_field_positions) {
            let bytecode_start = c.pos();
            c.bytes(&entry.bytecode);

            // bytecodeRelAddr is relative to the position of the field
            // itself, and both positions are chunk-local, so we can
            // compute and patch this immediately (no deferred Patch
            // needed - it never depends on the file's final layout).
            let rel = bytecode_start as i64 - rel_addr_field_pos as i64;
            c.set_u32(rel_addr_field_pos, rel as i32 as u32);
        }
    }

    c
}
