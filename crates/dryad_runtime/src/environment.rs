use std::collections::HashMap;
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub constants: HashMap<String, Value>,
    pub classes: HashMap<String, Value>,
    pub current_instance: Option<Value>,
    pub imported_modules: HashMap<String, HashMap<String, Value>>,
    pub call_stack_vars: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            constants: HashMap::new(),
            classes: HashMap::new(),
            current_instance: None,
            imported_modules: HashMap::new(),
            call_stack_vars: Vec::new(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_constant(&self, name: &str) -> Option<Value> {
        self.constants.get(name).cloned()
    }

    pub fn set_constant(&mut self, name: String, value: Value) {
        self.constants.insert(name, value);
    }

    pub fn get_class(&self, name: &str) -> Option<Value> {
        self.classes.get(name).cloned()
    }

    pub fn set_class(&mut self, name: String, value: Value) {
        self.classes.insert(name, value);
    }

    pub fn push_scope(&mut self) {
        self.call_stack_vars.push(self.variables.clone());
    }

    pub fn pop_scope(&mut self) -> bool {
        if let Some(saved) = self.call_stack_vars.pop() {
            self.variables = saved;
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.variables.clear();
        self.constants.clear();
        self.classes.clear();
        self.current_instance = None;
        self.imported_modules.clear();
        self.call_stack_vars.clear();
    }
}
