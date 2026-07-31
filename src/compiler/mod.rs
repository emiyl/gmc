pub mod ast;
pub mod bytecode;
pub mod compiler;
pub mod disassembler;
pub mod encoder;
pub mod instruction;
pub mod lexer;
pub mod parser;
pub mod resolver;

use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;
use resolver::Resolver;

pub struct Program {
    pub bytecode: bytecode::Bytecode,
    pub variables: Vec<resolver::Variable>,
    pub functions: Vec<resolver::Function>,
}

impl Program {
    pub fn new(
        instructions: Vec<instruction::Instruction>,
        variables: Vec<resolver::Variable>,
        functions: Vec<resolver::Function>,
    ) -> Self {
        let bytecode = encoder::encode(instructions);
        Self {
            bytecode,
            variables,
            functions,
        }
    }
}

fn create_program_from_gml(input: &str) -> Program {
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program_ast = parser.parse_program();
    log::debug!("Parsed AST: {:#?}", program_ast);

    let mut compiler = Compiler::new();
    compiler.compile_program(&program_ast);

    let mut resolver = Resolver::new();
    resolver.resolve(compiler.instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: &'static str,
        expected_bytecode: &'static [u32],
    }

    const TEST_CASES: &[TestCase] = &[
        TestCase {
            input: "a = 1;",
            expected_bytecode: &[
                0x840F0001, // PushI.e 1
                0x4525FFFA, // Pop.v.i
                0xA0000000, // Variable index 0
            ],
        },
        TestCase {
            input: "a = 1; b = 2; c = a + b;",
            expected_bytecode: &[
                0x840F0001, // PushI.e 1
                0x4525FFFA, // Pop.v.i
                0xA0000000, // Variable index 0
                0x840F0002, // PushI.e 2
                0x4525FFFA, // Pop.v.i
                0xA0000001, // Variable index 1
                0xC005FFFA, // Push.v.i
                0xA0000000, // Variable index 0
                0xC005FFFA, // Push.v.i
                0xA0000001, // Variable index 1
                0x0C550000, // Add
                0x4525FFFA, // Pop.v.i
                0xA0000002, // Variable index 2
            ],
        },
    ];

    #[test]
    fn test() {
        for test_case in TEST_CASES {
            let program = create_program_from_gml(test_case.input);
            let bytecode = &program.bytecode.data;
            let bytecode_as_u32_chunks = bytecode
                .chunks_exact(4)
                .map(|chunk| {
                    let mut array = [0u8; 4];
                    array.copy_from_slice(chunk);
                    u32::from_le_bytes(array)
                })
                .collect::<Vec<u32>>();
            assert_eq!(bytecode_as_u32_chunks, test_case.expected_bytecode);
        }
    }
}
