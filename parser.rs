use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use swc_common;
use swc_common::errors::ColorConfig;
use swc_common::errors::Diagnostic;
use swc_common::errors::DiagnosticBuilder;
use swc_common::errors::Emitter;
use swc_common::errors::Handler;
use swc_common::errors::HandlerFlags;
use swc_common::sync::Lrc;
use swc_common::FileName;
use swc_common::SourceMap;
use swc_ecmascript::ast::Program;
use swc_ecmascript::parser::lexer::Lexer;
use swc_ecmascript::parser::Capturing;
use swc_ecmascript::parser::Parser;
use swc_ecmascript::parser::StringInput;
use swc_ecmascript::parser::Syntax;

#[derive(Clone, Debug)]
pub struct DiagnosticBuffer {
    pub diagnostics: Vec<String>,
}

impl Error for DiagnosticBuffer {}

impl fmt::Display for DiagnosticBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.diagnostics.join(",");

        f.pad(&msg)
    }
}

impl DiagnosticBuffer {
    pub(crate) fn from_swc_error(error_buffer: ErrorBuffer, source_map: Lrc<SourceMap>) -> Self {
        let s = error_buffer.0.borrow().clone();

        let diagnostics = s
            .iter()
            .map(|d| {
                let mut msg = d.message();

                if let Some(span) = d.span.primary_span() {
                    let location = source_map.lookup_char_pos(span.lo());
                    let filename = match &location.file.name {
                        FileName::Custom(n) => n,
                        _ => unreachable!(),
                    };
                    msg = format!(
                        "{} at {}:{}:{}",
                        msg, filename, location.line, location.col_display
                    );
                }

                msg
            })
            .collect::<Vec<String>>();

        Self { diagnostics }
    }
}

#[derive(Clone)]
pub struct ErrorBuffer(Rc<RefCell<Vec<Diagnostic>>>);

impl ErrorBuffer {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![])))
    }
}

impl Emitter for ErrorBuffer {
    fn emit(&mut self, db: &DiagnosticBuilder) {
        self.0.borrow_mut().push((**db).clone());
    }
}

pub fn parse(src: &str) -> Program {
    let cm: Lrc<SourceMap> = Default::default();
    let buffered_error = ErrorBuffer::new();

    let handler = Handler::with_emitter_and_flags(
        Box::new(buffered_error.clone()),
        HandlerFlags {
            dont_buffer_diagnostics: true,
            can_emit_warnings: true,
            ..Default::default()
        },
    );

    let fm = cm.new_source_file(FileName::Custom("test.js".into()), src.into());

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);

    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_program()
        .map_err(|e| {
            e.into_diagnostic(&handler).emit();
            DiagnosticBuffer::from_swc_error(buffered_error.clone(), cm)
        })
        .expect("Failed to parse module.");
    module
}
