pub struct Env {
    function_store: Vec<String>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            function_store: Vec::new(),
        }
    }

    pub fn add(&mut self, id: String) {
        self.function_store.push(id);
    }

    pub fn has(&mut self, id: String) -> bool {
        self.function_store.contains(&id)
    }
}
