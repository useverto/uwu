use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::env;
use std::fs;
use uwu::parser::Parser;
use uwu::scanner::Scanner;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let source = fs::read_to_string(&args[1]).expect("Unable to read file");
        let mut scanner = Scanner::new();
        let ast = Parser::new(&args[1], &source).parse();
        scanner.scan(ast);
        let diagnostics = scanner.diagnostics();
        if diagnostics.len() > 0 {
            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();
            let file = SimpleFile::new(&args[1], source);
            for err in diagnostics {
                let diagnostic = Diagnostic::error()
                    .with_message("Error")
                    .with_labels(vec![
                        Label::primary((), err.loc.0..err.loc.1).with_message(&err.msg)
                    ])
                    .with_notes(vec![err.msg]);

                term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
            }
        }
    }
}
