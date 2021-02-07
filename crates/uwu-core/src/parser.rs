use crate::parser_diagnostics::DiagnosticBuffer;
use crate::parser_diagnostics::ErrorBuffer;
use swc_common;
use swc_common::errors::Handler;
use swc_common::errors::HandlerFlags;
use swc_common::sync::Lrc;
use swc_common::FileName;
use swc_common::SourceMap;
use swc_ecmascript::ast::Program;
use swc_ecmascript::parser::lexer::Lexer;
use swc_ecmascript::parser::Capturing;
use swc_ecmascript::parser::Parser as SwcParser;
use swc_ecmascript::parser::StringInput;
use swc_ecmascript::parser::Syntax;

pub struct Parser(FileName, String);

impl Parser {
    pub fn new(filename: &str, source: &str) -> Self {
        Self(FileName::Custom(filename.into()), source.into())
    }

    pub fn parse(&self) -> Program {
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

        let fm = cm.new_source_file(self.0.clone(), self.1.clone());

        let lexer = Lexer::new(
            Syntax::Es(Default::default()),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let capturing = Capturing::new(lexer);

        let mut parser = SwcParser::new_from(capturing);

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
}
