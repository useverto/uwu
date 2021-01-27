use std::env;
use std::fs;
use uwu::compiler::Compiler;
use uwu::parser::Parser;
use uwu::tokenizer::Lexer;

fn c(source: &str) -> String {
    let mut parser = Parser::new(Lexer::new(source));
    let ast = parser.parse();
    let compiler = Compiler::new(ast);
    compiler.compile()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let source = fs::read_to_string(&args[1]).expect("Unable to read file");
        println!("{}", c(&source));
    }
}
