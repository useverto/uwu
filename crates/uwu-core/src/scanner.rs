use crate::context::Context;
use std::cell::RefCell;
use std::rc::Rc;
use swc_common::BytePos;
use swc_ecmascript::{
    ast::{
        CallExpr, ClassExpr, Expr, ExprOrSuper, FnDecl, FnExpr, Function, Ident, MemberExpr, Pat,
        Program, VarDecl,
    },
    utils::{find_ids, ident::IdentLike, Id},
    visit::{noop_visit_type, Node, Visit, VisitWith},
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
        if !ctx.scope.contains(ident.as_ref().to_string()) {
            ctx.diagnostics.push(Diagnostic {
                kind: ScannerErrorKind::ItemNotFound,
                loc: (lo as usize, hi as usize),
                msg: "Item not found in scope.".to_string(),
            });
        }
    }

    // Adapted from https://github.com/denoland/deno_lint/blob/64232f8586a1c0c51a175ccd545344adf5d96def/src/scopes.rs#L159
    fn visit_var_decl(&mut self, n: &VarDecl, _parent: &dyn Node) {
        n.decls.iter().for_each(|v| {
            v.init.visit_with(n, self);
            let mut ctx = self.context.borrow_mut();
            if let Some(expr) = &v.init {
                if let Expr::Class(ClassExpr {
                    ident: Some(class_name),
                    ..
                }) = &**expr
                {
                    if let Pat::Ident(var_name) = &v.name {
                        if var_name.sym == class_name.sym {
                            ctx.scope.vars.push(class_name.as_ref().to_string());
                            return;
                        }
                    }
                }
            }

            let ids: Vec<Id> = find_ids(&v.name);
            for id in ids {
                ctx.scope.vars.push(id.0.to_string());
            }
        })
    }

    fn visit_fn_decl(&mut self, n: &FnDecl, _parent: &dyn Node) {
        let mut ctx = self.context.borrow_mut();
        ctx.scope.functions.push(n.ident.as_ref().to_string());
        drop(ctx);
        n.function.visit_with(n, self);
    }

    fn visit_fn_expr(&mut self, n: &FnExpr, _: &dyn Node) {
        let mut ctx = self.context.borrow_mut();
        if let Some(ident) = &n.ident {
            ctx.scope.functions.push(ident.as_ref().to_string());
        }
        drop(ctx);
        n.function.visit_with(n, self);
    }

    fn visit_function(&mut self, n: &Function, _: &dyn Node) {
        n.decorators.visit_with(n, self);
        n.params.visit_with(n, self);

        match &n.body {
            Some(s) => s.stmts.visit_with(n, self),
            None => {}
        }
    }
}
