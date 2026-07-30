mod ast;
mod bytecode;
mod compiler;
mod data_win;
mod disassembler;
mod encoder;
mod instruction;
mod lexer;
mod parser;
mod resolver;
mod wad_layout;

use clap::Parser as ClapParser;
use compiler::Compiler;
use env_logger::Builder;
use lexer::Lexer;
use log::LevelFilter;
use parser::Parser;
use resolver::Resolver;
use std::io::Write;

use crate::disassembler::print_disassembly;

#[derive(ClapParser)]
#[command(name = "gmlc")]
#[command(about = "GameMaker Language compiler")]
struct Args {
    /// Input file
    input: String,

    /// Output file
    #[arg(short, long)]
    output: Option<String>,
}

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

fn main() {
    let args: Args = ClapParser::parse();
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
    let program = create_program_from_gml(&input);

    print_disassembly(&program.bytecode);

    if args.output.is_none() {
        println!("No output file specified. Use -o <file> to specify an output file.");
    } else {
        let output_data = data_win::build_data_win(&args.input, program);
        let output_file = args.output.as_ref().unwrap();
        std::fs::write(output_file, output_data).expect("Failed to write output file");
    }
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
