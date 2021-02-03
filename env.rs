use std::collections::HashMap;

use crate::{ast::Literal, ltype, types::Type};

pub struct Env {
    function_store: Vec<String>,
    type_store: HashMap<String, Type>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            function_store: vec![],
            type_store: HashMap::new(),
        }
    }

    pub fn add_fn(&mut self, id: String) {
        self.function_store.push(id);
    }

    pub fn has_fn(&mut self, id: String) -> bool {
        self.function_store.contains(&id)
    }

    pub fn addt(&mut self, id: &str, t: &Literal) {
        self.type_store.insert(id.to_string(), ltype!(t));
    }

    pub fn sett(&mut self, id: &str, t: Type) {
        self.type_store.insert(id.to_string(), t);
    }

    pub fn checkt(&mut self, id: &str, t: &Type) -> bool {
        match self.type_store.get(id) {
            Some(t2) => t2 == t,
            None => false,
        }
    }

    pub fn gett(&mut self, id: String) -> &Type {
        match self.type_store.get(&id) {
            Some(t) => t,
            None => &Type::Unknown,
        }
    }
}
