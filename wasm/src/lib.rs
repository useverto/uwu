use uwu::compiler::Compiler;
use uwu::create_diagnostic;
use uwu::parser::{ParseError, Parser};
use uwu::compiler::CompilerError;
use uwu::tokenizer::Lexer;
use wasm_bindgen::prelude::*;
use codespan_reporting::term::termcolor::{Color, ColorSpec, StandardStream, WriteColor, Buffer};
use codespan_reporting::term::{self, ColorArg};
use codespan_reporting::term::emit;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use std::io::{self, Write};
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum Error {
    ParseError(Vec<ParseError>),
    CompilerError(CompilerError),
}

fn c(source: &str) -> Result<String, Error> {
    let mut parser = Parser::new(Lexer::new(source));
    let ast = parser.parse();
    let errs = parser.get_errors();
    if errs.len() > 0 {
        return Err(Error::ParseError(errs));
    }
    let compiler = Compiler::new(ast);
    Ok(compiler.compile().map_err(|c| Error::CompilerError(c))?)
}


#[wasm_bindgen(catch)]
pub fn compile(s: String) -> Result<String, JsValue> {

    let config = codespan_reporting::term::Config::default();
    let mut buffer = Buffer::no_color();
    match c(&s) {
        Ok(r) => return Ok(r),
        Err(e) => {
            let file = SimpleFile::new("<repl>", s);
            match e {
                Error::ParseError(e) => {
                    for error in e {
                        let diagnostic = Diagnostic::error()
                            .with_message("Error")
                            .with_labels(vec![Label::primary(
                                (),
                                error.current_token.loc..error.current_token.loc,
                            )
                            .with_message("parse error")])
                            .with_notes(vec![error.msg]);

                        emit(&mut buffer, &config, &file, &diagnostic).unwrap();
                    }
                }
                Error::CompilerError(e) => {
                    let diagnostic =
                        Diagnostic::error().with_message("Error").with_labels(vec![
                            Label::primary((), e.loc..e.loc).with_message(format!("{}", e)),
                        ]);

                    emit(&mut buffer, &config, &file, &diagnostic).unwrap();
                }
            }
        }
    }
    Ok(std::str::from_utf8(&buffer.as_slice()).unwrap().to_string())
}
