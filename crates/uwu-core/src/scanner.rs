use crate::context::Context;
use std::cell::RefCell;
use std::rc::Rc;
use swc_common::BytePos;
use swc_ecmascript::{
    ast::{CallExpr, Expr, ExprOrSuper, Ident, MemberExpr, Program},
    visit::{noop_visit_type, Node, Visit},
};

#[derive(Debug, Clone)]
pub enum ScannerErrorKind {
    ComputedMemberExpr,
    ItemNotFound,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: ScannerErrorKind,
    pub loc: (usize, usize),
    pub msg: String,
}

#[derive(Debug, Clone)]
pub struct Scanner {
    context: Rc<RefCell<Context>>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            context: Rc::new(RefCell::new(Default::default())),
        }
    }

    pub fn scan(&mut self, program: Program) {
        self.visit_program(&program, &program);
    }

    pub fn diagnostics(&mut self) -> Vec<Diagnostic> {
        self.context.borrow().diagnostics.clone()
    }

    fn visit_exprorsuper(&mut self, expr: ExprOrSuper, parent: &dyn Node) {
        if let Some(expr) = expr.expr() {
            match *expr {
                Expr::Member(member_expr) => {
                    self.check_member_expr(&member_expr);
                    self.visit_exprorsuper(member_expr.obj, parent);
                }
                _ => (),
            }
        }
    }

    fn check_member_expr(&mut self, bin_expr: &MemberExpr) {
        if bin_expr.computed {
            let BytePos(lo) = bin_expr.span.lo();
            let BytePos(hi) = bin_expr.span.hi();
            self.context.borrow_mut().diagnostics.push(Diagnostic {
                kind: ScannerErrorKind::ComputedMemberExpr,
                loc: (lo as usize, hi as usize),
                msg: "Computed member expression are not allowed.".to_string(),
            });
        }
    }
}

impl Visit for Scanner {
    noop_visit_type!();
    /// Implements scanning index/member expressions as callee call expressions.
    fn visit_call_expr(&mut self, call_expr: &CallExpr, parent: &dyn Node) {
        self.visit_exprorsuper(call_expr.callee.clone(), parent);
    }

    fn visit_ident(&mut self, ident: &Ident, _parent: &dyn Node) {
        let mut ctx = self.context.borrow_mut();
        let BytePos(lo) = ident.span.lo();
        let BytePos(hi) = ident.span.hi();
        if !ctx.scope.has_global(ident.as_ref().to_string()) {
            ctx.diagnostics.push(Diagnostic {
                kind: ScannerErrorKind::ItemNotFound,
                loc: (lo as usize, hi as usize),
                msg: "Item not found in scope.".to_string(),
            });
        }
    }
}
