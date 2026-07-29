mod ast;
mod bytecode;
mod compiler;
mod disassembler;
mod encoder;
mod instruction;
mod lexer;
mod parser;
mod resolver;

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

fn main() {
    let args: Args = ClapParser::parse();
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    log::debug!("Parsed AST: {:#?}", program);

    let mut compiler = Compiler::new();
    compiler.compile_program(&program);

    let mut resolver = Resolver::new();
    let resolved = resolver.resolve(compiler.instructions);

    let bytecode = encoder::encode(resolved);
    print_disassembly(&bytecode);

    if args.output.is_none() {
        println!("No output file specified. Use -o <file> to specify an output file.");
    } else {
        let output_file = args.output.unwrap_or_else(|| "output.gmlc".to_string());
        std::fs::write(output_file, bytecode.data).expect("Failed to write output file");
    }
}
