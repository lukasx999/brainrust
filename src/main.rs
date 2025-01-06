mod parser;
use parser::Parser;

mod interpreter;
use interpreter::Interpreter;

mod compiler;
use compiler::Compiler;



fn print_usage(args: &Vec<String>) {
    eprintln!("Usage: {} <file.bf> <run/comp>", args[0]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        print_usage(&args);
        std::process::exit(1);
    }

    let filename = args[1].clone();
    let mode     = args[2].clone();

    let src: String = std::fs::read_to_string(filename)?;
    let tokens = Parser::new(src).parse();

    match mode.as_str() {
        "comp" => Compiler::new(tokens, "out.s".to_owned()).compile(),
        "run"  => Interpreter::new(tokens).run(),
        m => {
            eprintln!("Unknown mode: {m}");
            std::process::exit(1);
        }
    }

    Ok(())
}
