use std::env;
use std::fs;
use uwu::compiler::{Compiler, CompilerError};
use uwu::parser::{ParseError, Parser};
use uwu::tokenizer::Lexer;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

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

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        let source = fs::read_to_string(&args[1]).expect("Unable to read file");
        match c(&source) {
            Ok(r) => println!("{}", r),
            Err(e) => {
                let file = SimpleFile::new("<repl>", source);
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

                            term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
                        }
                    }
                    Error::CompilerError(e) => {
                        let diagnostic =
                            Diagnostic::error().with_message("Error").with_labels(vec![
                                Label::primary((), e.loc..e.loc).with_message(format!("{}", e)),
                            ]);

                        term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
                    }
                }
            }
        }
    }
}
