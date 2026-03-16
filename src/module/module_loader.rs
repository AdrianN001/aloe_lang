use std::collections::HashMap;

use crate::module::{Module, ModuleRef, module_error::ModuleError};

pub struct ModuleLoader {
    cache: HashMap<String, ModuleRef>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn get(&self, module_name: &str) -> Option<ModuleRef> {
        self.cache.get(module_name).cloned()
    }

    pub fn set(&mut self, mod_ref: ModuleRef) {
        let key = {
            let borrowed = mod_ref.borrow();

            borrowed.as_abs_path().clone()
        };

        self.cache.insert(key, mod_ref.clone());
    }

    pub fn import_module(&mut self, module_name: &str) -> Result<ModuleRef, ModuleError> {
        if let Some(module) = self.get(module_name) {
            return Ok(module.clone());
        }

        let module = Module::new(module_name.to_string())?.to_reference();

        self.set(module.clone());

        {
            let mut borrow = module.borrow_mut();
            borrow.execute(self);
        }

        Ok(module)
    }
}
