mod ast;
mod compiler;
mod debug;
mod lexer;
mod parser;
mod value;
mod vm;

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
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    for decl in &ast {
        debug::traverse_print(decl);
    }

    // Compile the AST
    let mut compiler = compiler::Compiler::new(ast, vm::VM::new());
    compiler.compile();

    // Run the VM
    compiler.vm.execute();

    // Test program
    // Equivalent to:
    // proc main do
    //     if x = 1 then
    //         print x
    //     else
    //        set x -> x - 1
    //        call main
    //     end
    // end
    //
    // set x -> 10
    // call main
    // let procedure = vec![
    //     vm::OpCode::GetGlobal("x".to_string()),
    //     vm::OpCode::Constant(vm.write_constant(value::Value::Number(1.0))),
    //     vm::OpCode::Equal,
    //     vm::OpCode::JumpForwardIfFalse(4),
    //     vm::OpCode::GetGlobal("x".to_string()),
    //     vm::OpCode::Print,
    //     vm::OpCode::JumpForward(6),
    //     vm::OpCode::GetGlobal("x".to_string()),
    //     vm::OpCode::Constant(vm.write_constant(value::Value::Number(1.0))),
    //     vm::OpCode::Subtract,
    //     vm::OpCode::SetGlobal("x".to_string()),
    //     vm::OpCode::ProcedureCall("main".to_string()),
    // ];

    // vm.write_procedure("main".to_string(), procedure);

    // let idx = vm.write_constant(value::Value::Number(10.0));
    // vm.write_op(vm::OpCode::Constant(idx));
    // vm.write_op(vm::OpCode::SetGlobal("x".to_string()));
    // vm.write_op(vm::OpCode::ProcedureCall("main".to_string()));

    // vm.execute();
}
