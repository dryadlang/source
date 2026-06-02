use std::collections::HashMap;
use crate::native_modules::{NativeModuleManager, NativeFunction};
use crate::value::Value;
use crate::heap::Heap;
use crate::errors::RuntimeError;

pub struct NativeRegistry {
    pub manager: NativeModuleManager,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            manager: NativeModuleManager::new(),
        }
    }

    pub fn call_native(
        &self,
        name: &str,
        args: &[Value],
        heap: &mut Heap,
    ) -> Result<Value, RuntimeError> {
        if let Some(func) = self.manager.get_function(name) {
            func(args, &self.manager, heap)
        } else {
            Err(RuntimeError::Generic(format!("Native function {} not found", name)))
        }
    }

    pub fn activate_module(&mut self, name: &str) -> bool {
        self.manager.activate_category(name).is_ok()
    }

    pub fn is_module_active(&self, name: &str) -> bool {
        self.manager.is_category_active(name)
    }

    pub fn list_active_functions(&self) -> Vec<String> {
        self.manager.list_active_functions()
    }
}
