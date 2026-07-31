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
