// Adopted from deno_lint's parser.rs
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use swc_common::errors::Diagnostic;
use swc_common::errors::DiagnosticBuilder;
use swc_common::errors::Emitter;
use swc_common::sync::Lrc;
use swc_common::FileName;
use swc_common::SourceMap;

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
    pub fn from_swc_error(error_buffer: ErrorBuffer, source_map: Lrc<SourceMap>) -> Self {
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
