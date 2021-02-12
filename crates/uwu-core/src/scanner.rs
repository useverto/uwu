use crate::{context::Context, scope::Scope};
#[cfg(feature = "serde_json")]
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use swc_common::{BytePos, Span};
use swc_ecmascript::{
    ast::{
        AssignPatProp, BlockStmt, CallExpr, ClassDecl, ClassExpr, ClassProp, Expr, ExprOrSuper,
        FnDecl, FnExpr, Function, Ident, MemberExpr, Param, Pat, Program, Prop, UnaryExpr, UnaryOp,
        VarDecl, VarDeclarator,
    },
    utils::{find_ids, Id},
    visit::{noop_visit_type, Node, Visit, VisitWith},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(Serialize, Deserialize))]
pub enum ScannerErrorKind {
    ComputedMemberExpr,
    ItemNotFound,
    ComputedCallExpr,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(Serialize, Deserialize))]
pub struct Diagnostic {
    pub kind: ScannerErrorKind,
    pub loc: (usize, usize),
    pub msg: String,
}

#[derive(Debug, Clone)]
pub struct Scanner {
    context: Rc<RefCell<Context>>,
}

impl Diagnostic {
    pub fn new(kind: ScannerErrorKind, loc: (usize, usize), msg: String) -> Self {
        Self { kind, loc, msg }
    }
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            context: Rc::new(RefCell::new(Default::default())),
        }
    }

    pub fn from_scope(scope: Scope) -> Self {
        Self {
            context: Rc::new(RefCell::new(Context::new(scope)))
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
                Expr::Ident(ident) => {
                    self.check(&ident);
                }
                Expr::This(expr) => self.error_computed_expr(expr.span),
                Expr::Array(_) => {}
                Expr::Object(expr) => self.error_computed_expr(expr.span),
                Expr::Fn(expr) => self.error_computed_expr(expr.function.span),
                Expr::Unary(expr) => self.error_computed_expr(expr.span),
                Expr::Update(expr) => self.error_computed_expr(expr.span),
                Expr::Bin(expr) => self.error_computed_expr(expr.span),
                Expr::Assign(expr) => self.error_computed_expr(expr.span),
                Expr::Cond(expr) => self.error_computed_expr(expr.span),
                Expr::Call(expr) => self.error_computed_expr(expr.span),
                Expr::New(expr) => self.error_computed_expr(expr.span),
                Expr::Seq(expr) => self.error_computed_expr(expr.span),
                // Hmmm
                Expr::Lit(_) => {}
                Expr::Tpl(_) => {}
                Expr::TaggedTpl(_) => {}
                Expr::Arrow(expr) => self.error_computed_expr(expr.span),
                Expr::Class(expr) => self.error_computed_expr(expr.class.span),
                Expr::Yield(expr) => self.error_computed_expr(expr.span),
                Expr::MetaProp(expr) => self.error_computed_expr(expr.prop.span),
                Expr::Await(expr) => self.error_computed_expr(expr.span),
                Expr::Paren(expr) => self.error_computed_expr(expr.span),
                Expr::PrivateName(expr) => self.error_computed_expr(expr.span),
                Expr::OptChain(expr) => self.error_computed_expr(expr.span),
                Expr::Invalid(_) => {}
                _ => {}
            }
        }
    }

    fn error_computed_expr(&mut self, span: Span) {
        let BytePos(lo) = span.lo();
        let BytePos(hi) = span.hi();
        self.context.borrow_mut().diagnostics.push(Diagnostic {
            kind: ScannerErrorKind::ComputedCallExpr,
            loc: (lo as usize, hi as usize),
            msg: "Computed call expressions are not allowed.".to_string(),
        });
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

    fn check(&mut self, ident: &Ident) {
        if ident.sym == *"arguments" {
            return;
        }

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
}

impl Visit for Scanner {
    noop_visit_type!();
    /// Implements scanning index/member expressions as callee call expressions.
    fn visit_call_expr(&mut self, call_expr: &CallExpr, parent: &dyn Node) {
        self.visit_exprorsuper(call_expr.callee.clone(), parent);
    }

    fn visit_unary_expr(&mut self, e: &UnaryExpr, _: &dyn Node) {
        if e.op == UnaryOp::TypeOf {
            return;
        }

        e.visit_children_with(self);
    }

    fn visit_expr(&mut self, e: &Expr, _: &dyn Node) {
        e.visit_children_with(self);

        if let Expr::Ident(ident) = e {
            self.check(ident)
        }
    }

    fn visit_class_prop(&mut self, p: &ClassProp, _: &dyn Node) {
        p.value.visit_with(p, self)
    }

    fn visit_prop(&mut self, p: &Prop, _: &dyn Node) {
        p.visit_children_with(self);

        if let Prop::Shorthand(i) = &p {
            self.check(i);
        }
    }

    fn visit_pat(&mut self, p: &Pat, _: &dyn Node) {
        if let Pat::Ident(i) = p {
            self.check(i);
        } else {
            p.visit_children_with(self);
        }
    }

    fn visit_assign_pat_prop(&mut self, p: &AssignPatProp, _: &dyn Node) {
        self.check(&p.key);
        p.value.visit_with(p, self);
    }

    /// Check declarator init expressions.
    /// Example:
    ///
    /// let _ = (() => [eval])()[0];
    ///         ^^^^^^^^^^^^^^ Computed call expressions are not allowed.
    ///
    fn visit_var_declarator(&mut self, n: &VarDeclarator, _parent: &dyn Node) {
        if let Some(init) = &n.init {
            init.visit_with(n, self);
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

    fn visit_class_decl(&mut self, n: &ClassDecl, _: &dyn Node) {
        let mut ctx = self.context.borrow_mut();
        ctx.scope.functions.push(n.ident.as_ref().to_string());
        drop(ctx);
        n.class.visit_with(n, self);
    }

    fn visit_block_stmt(&mut self, n: &BlockStmt, _: &dyn Node) {
        n.stmts.visit_with(n, self);
    }

    fn visit_param(&mut self, p: &Param, _: &dyn Node) {
        let ids: Vec<Id> = find_ids(&p.pat);
        let mut ctx = self.context.borrow_mut();
        for id in ids {
            ctx.scope.vars.push(id.0.to_string());
        }
    }
}
