
use swc_ecmascript::utils::Id;

#[derive(Debug)]
pub struct Var {
  path: Vec<ScopeKind>,
  kind: BindingKind,
}

impl Var {
  /// Empty path means root scope.
  #[allow(dead_code)]
  pub fn path(&self) -> &[ScopeKind] {
    &self.path
  }

  pub fn kind(&self) -> BindingKind {
    self.kind
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BindingKind {
  Var,
  Const,
  Let,
  Function,
  Param,
  Class,
  CatchClause,
  Import,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ScopeKind {
  // Module,
  Arrow,
  Function,
  Block,
  Loop,
  Class,
  Switch,
  With,
  Catch,
}

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<Id, Var>,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Self { vars: vec![] }
    }

    pub fn new_with_globals(globals: Vec<String>) -> Self {
        Self { globals }
    }

    pub fn has_global(&self, source: String) -> bool {
        self.globals.contains(&source)
    }
}
