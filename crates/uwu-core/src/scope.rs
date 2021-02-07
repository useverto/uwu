// XXX: Please use swc_ecmascript::utils::Id or Ident
#[derive(Debug, Clone)]
pub struct Scope {
    globals: Vec<String>,
    pub vars: Vec<String>,
    pub functions: Vec<String>,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            globals: vec![],
            vars: vec![],
            functions: vec![],
        }
    }

    pub fn new_with_globals(globals: Vec<String>) -> Self {
        Self {
            globals,
            vars: vec![],
            functions: vec![],
        }
    }

    pub fn has_global(&self, ident: String) -> bool {
        self.globals.contains(&ident)
    }

    pub fn has_fn(&self, ident: String) -> bool {
        self.functions.contains(&ident)
    }

    pub fn contains(&self, ident: String) -> bool {
        self.vars.contains(&ident) || self.functions.contains(&ident)
    }
}
