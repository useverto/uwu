use crate::scanner::Diagnostic;
use crate::scope::Scope;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    pub(crate) scope: Scope,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

impl Default for Context {
    fn default() -> Self {
        Context::new(Scope::new())
    }
}

impl Context {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            diagnostics: vec![],
        }
    }
}
