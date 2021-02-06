use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::env;
use std::fs;
use swc_ecmascript::visit::Visit;
use uwu::parser::parse;
use uwu::scanner::Scanner;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        let source = fs::read_to_string(&args[1]).expect("Unable to read file");
        let mut scanner = Scanner {};
        let ast = parse(&source);
        scanner.visit_program(&ast, &ast);
    }
}
