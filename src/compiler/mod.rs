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

pub fn create_program_from_gml(input: &str) -> Program {
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
        TestCase {
            input: "a = 1; a += 2;",
            expected_bytecode: &[
                0x840F0001, // PushI.e 1
                0x4525FFFA, // Pop.v.i
                0xA0000000, // Variable index 0
                0xC005FFFA, // Push.v.i
                0xA0000000, // Variable index 0
                0x840F0002, // PushI.e 2
                0x0C520000, // Add.v.i
                0x4525FFFA, // Pop.v.i
                0xA0000000, // Variable index 0
            ],
        },
        TestCase {
            input: "a = true;",
            expected_bytecode: &[
                0xC0040001, // Push.b true
                0x4545FFFA, // Pop.v.b
                0xA0000000, // Variable index 0
            ],
        },
        TestCase {
            input: "a = 1; a++; ++a; a--; --a;",
            expected_bytecode: &[
                0x840F0001, // PushI.e 1
                0x4525FFFA, // Pop.v.i
                0xA0000000, // Variable index 0
                0xC005FFFA, // Push.v.i
                0xA0000000, // Variable index 0
                0xC00F0001, // Push.e 1
                0x0C520000, // Add.i.v
                0x4555FFFA, // Pop.v.v
                0xA0000000, // Variable index 0
                0xC005FFFA, // Push.v
                0xA0000000, // Variable index 0
                0xC00F0001, // Push.e 1
                0x0C520000, // Add.i.v
                0x4555FFFA, // Pop.v
                0xA0000000, // Variable index 0
                0xC005FFFA, // Push.v
                0xA0000000, // Variable index 0
                0xC00F0001, // Push.e 1
                0x0D520000, // Sub.i.v
                0x4555FFFA, // Pop.v
                0xA0000000, // Variable index 0
                0xC005FFFA, // Push.v
                0xA0000000, // Variable index 0
                0xC00F0001, // Push.e 1
                0x0D520000, // Sub.i.v
                0x4555FFFA, // Pop.v.v
                0xA0000000, // Variable index 0
            ],
        },
        TestCase {
            input: "msg = \"hello\"; show_debug_message(msg);",
            expected_bytecode: &[
                0xC0060000, // Push.s
                0x00000000, // String reference placeholder (patched in CODE chunk)
                0x4565FFFA, // Pop.v.s
                0xA0000000, // Variable index 0
                0xC005FFFA, // Push.v
                0xA0000000, // Variable index 0
                0xD9020001, // Call.i args=1
                0x00000000, // Function index 0
                0x9E050000, // PopZ
            ],
        },
        TestCase {
            input: "for (i = 0; i < 10; i += 1) {}",
            expected_bytecode: &[
                0x840F0000, // PushI.e 0
                0x4525FFFA, // Pop.v.i
                0xA0000000, // Variable index 0
                0xC005FFFA, // Push.v.i
                0xA0000000, // Variable index 0
                0x840F000A, // PushI.e 10
                0x15520100, // Cmp.i.v LT
                0xB8000008, // BranchFalse offset=8
                0xC005FFFA, // Push.v.i
                0xA0000000, // Variable index 0
                0xC00F0001, // Push.e 1
                0x0C520000, // Add.v.i
                0x4555FFFA, // Pop.v.v
                0xA0000000, // Variable index 0
                0xB67FFFF5, // Branch offset=-44 bytes
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
