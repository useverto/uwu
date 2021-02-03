use std::env;
use std::fs;
use uwu::compiler::Compiler;
use uwu::create_diagnostic;
use uwu::parser::Parser;
use uwu::tokenizer::Lexer;

fn c(source: &str) -> Result<String, String> {
    let mut parser = Parser::new(Lexer::new(source));
    let ast = parser.parse();
    let errs = parser.get_errors();
    if errs.len() > 0 {
        let e = &errs[0];
        return Err(create_diagnostic!(
            "main.uwu",
            source.chars().nth(e.current_token.loc - 1).unwrap(),
            [e.current_token.loc, e.current_token.loc],
            e.msg,
            e.msg
        ));
    }
    let compiler = Compiler::new(ast);
    Ok(compiler.compile().unwrap())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let source = fs::read_to_string(&args[1]).expect("Unable to read file");
        match c(&source) {
            Ok(r) => println!("{}", r),
            Err(e) => println!("{}", e),
        }
    }
}
