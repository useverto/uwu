use uwu::compiler::Compiler;
use uwu::create_diagnostic;
use uwu::parser::Parser;
use uwu::tokenizer::Lexer;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    Ok(compiler.compile())
}

#[wasm_bindgen]
pub fn compile(s: String) -> String {
    c(&s).unwrap()
}
