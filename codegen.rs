pub trait BaseGenerator {
    fn new() -> Self;
    fn generate(&mut self) -> String;
}

enum FunctionCursorState {
    None,
    Name,
    Params,
    Block,
}

pub struct FunctionGenerator {
    code: String,
    state: FunctionCursorState,
}

impl BaseGenerator for FunctionGenerator {
    fn new() -> Self {
        Self {
            code: String::from("function"),
            state: FunctionCursorState::None,
        }
    }

    fn generate(&mut self) -> String {
        self.code.clone()
    }
}

impl FunctionGenerator {
    pub fn set_name(&mut self, name: String) {
        // function <name>
        self.code.push(' ');
        self.code.push_str(&name);
        self.state = FunctionCursorState::Name;
    }

    pub fn set_param(&mut self, param: String) {
        match self.state {
            // function <name>(<param1>
            FunctionCursorState::Name => {
                self.code.push('(');
                self.state = FunctionCursorState::Params;
                self.code.push_str(&param);
            }
            // function <name>(<params1>,<param2>
            FunctionCursorState::Params => {
                self.code.push(',');
                self.code.push_str(&param);
            }
            _ => (),
        }
    }

    pub fn set_block(&mut self, block: String) {
        match self.state {
            // function <name>(<param1>,<param2>) {
            //   <block>
            // };
            FunctionCursorState::Params => {
                self.code.push_str(") {\n");
                self.code.push_str(&block);
                self.code.push_str("}\n");
                self.state = FunctionCursorState::Block;
            }
            // set_param was never called.
            FunctionCursorState::Name => {
                self.code.push_str("() {\n");
                self.code.push_str(&block);
                self.code.push_str("}\n");
                self.state = FunctionCursorState::Block;
            }
            _ => (),
        }
    }
}
