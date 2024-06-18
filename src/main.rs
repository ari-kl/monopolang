mod lexer;

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

    for token in tokens {
        println!("{:?}", token);
    }
}
