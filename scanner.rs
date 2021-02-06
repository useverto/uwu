use swc_common::{
    self,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    FileName, SourceMap,
};
use swc_ecmascript::{
    ast::{MemberExpr, Program},
    parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax},
    visit::{noop_visit_type, Node, Visit},
};

pub struct Scanner;

impl Visit for Scanner {
    noop_visit_type!();
    fn visit_member_expr(&mut self, bin_expr: &MemberExpr, parent: &dyn Node) {
        if bin_expr.computed {
            println!("HAHAHAHHA");
        }
    }
}
