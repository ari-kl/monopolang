use ast::Declaration;
use parser::Parser;

mod ast;
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

    // for decl in &ast {
    //     traverse_print(decl);
    // }
}

// pub fn traverse_print(decl: &Declaration) {
//     match decl {
//         Declaration::Variable(name, initializer) => {
//             println!("Let {}", name);
//             traverse_print_expression(initializer, 2);
//         }
//         Declaration::Statement(statement) => {
//             traverse_print_statement(statement, 0);
//         }
//     }
// }

// pub fn traverse_print_statement(statement: &ast::Statement, indent: usize) {
//     match statement {
//         ast::Statement::Print(expr) => {
//             println!("{}Print", " ".repeat(indent));
//             traverse_print_expression(expr, indent + 2);
//         }
//         ast::Statement::Expression(expr) => {
//             traverse_print_expression(expr, indent);
//         }
//         ast::Statement::Block(decls) => {
//             println!("{}Block", " ".repeat(indent));
//             for decl in decls {
//                 traverse_print(decl);
//             }
//         }
//         ast::Statement::If(condition, then_branch) => {
//             println!("{}If", " ".repeat(indent));
//             traverse_print_expression(condition, indent + 2);
//             println!("{}Then", " ".repeat(indent));
//             traverse_print_statement(then_branch, indent + 2);
//         }
//     }
// }

// pub fn traverse_print_expression(expr: &ast::Expression, indent: usize) {
//     match expr {
//         ast::Expression::Boolean(value) => {
//             println!("{}Boolean({})", " ".repeat(indent), value);
//         }
//         ast::Expression::Number(value) => {
//             println!("{}Number({})", " ".repeat(indent), value);
//         }
//         ast::Expression::String(value) => {
//             println!("{}String({})", " ".repeat(indent), value);
//         }
//         ast::Expression::Void => {
//             println!("{}Void", " ".repeat(indent));
//         }
//         ast::Expression::Variable(name) => {
//             println!("{}Variable({})", " ".repeat(indent), name);
//         }
//         ast::Expression::Unary(operator, right) => {
//             println!("{}Unary({:?})", " ".repeat(indent), operator);
//             traverse_print_expression(right, indent + 2);
//         }
//         ast::Expression::Binary(operator, left, right) => {
//             println!("{}Binary({:?})", " ".repeat(indent), operator);
//             traverse_print_expression(left, indent + 2);
//             traverse_print_expression(right, indent + 2);
//         }
//         ast::Expression::Logical(operator, left, right) => {
//             println!("{}Logical({:?})", " ".repeat(indent), operator);
//             traverse_print_expression(left, indent + 2);
//             traverse_print_expression(right, indent + 2);
//         }
//     }
// }
