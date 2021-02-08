use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::emit;
use codespan_reporting::term::termcolor::{Buffer, Color, ColorSpec, StandardStream, WriteColor};
use codespan_reporting::term::{self, ColorArg};
use std::io::{self, Write};
use uwu::parser::Parser;
use uwu::scanner::Scanner;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(catch)]
pub fn scan(source: String) -> Result<String, JsValue> {
    let config = codespan_reporting::term::Config::default();
    let mut buffer = Buffer::no_color();
    let mut scanner = Scanner::new();
    let ast = Parser::new("<repl>", &source).parse();
    scanner.scan(ast);
    let diagnostics = scanner.diagnostics();
    if diagnostics.len() > 0 {
        let file = SimpleFile::new("<repl>", source);
        for err in diagnostics {
            let diagnostic = Diagnostic::error()
                .with_message("Error")
                .with_labels(vec![
                    Label::primary((), err.loc.0..err.loc.1).with_message(&err.msg)
                ])
                .with_notes(vec![err.msg]);

            emit(&mut buffer, &config, &file, &diagnostic).unwrap();
            return Ok(std::str::from_utf8(&buffer.as_slice()).unwrap().to_string());
        }
    }
    Ok("true".to_string())
}
