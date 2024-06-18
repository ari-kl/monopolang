use ast::Declaration;
use parser::Parser;

mod ast;
mod debug;
mod lexer;
mod parser;

fn main() {
    // Read the source code from a file
    // File given by command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: monopolang [file]");
        std::process::exit(1);
    }

    let source = std::fs::read_to_string(&args[1]).expect("Failed to read file");

    // Create a lexer
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.scan_tokens();

    // Create a parser
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    for decl in &ast {
        debug::traverse_print(decl);
    }
}
