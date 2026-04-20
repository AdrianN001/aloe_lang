use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::module::{Module, ModuleRef, module_error::ModuleError};

#[derive(Default)]
pub struct ModuleLoader {
    cache: HashMap<String, ModuleRef>,
    pub root_file: PathBuf,
}

impl ModuleLoader {
    pub fn new(root_file: &str) -> Self {
        Self {
            cache: HashMap::new(),
            root_file: Path::new(root_file).to_path_buf(),
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
        let root_dir = self.root_file.parent().unwrap();
        let abs_path_of_module = root_dir.join(module_name);

        let abs_path_str = match abs_path_of_module.to_str() {
            Some(path_str) => path_str,
            None => return Err(ModuleError::new(module_name, "module cannot be loaded")),
        };

        if let Some(module) = self.get(abs_path_str) {
            return Ok(module.clone());
        }

        let module = Module::new(abs_path_str.to_string())?.to_reference();

        self.set(module.clone());

        {
            let mut borrow = module.borrow_mut();
            borrow.execute(self).unwrap();
        }

        Ok(module)
    }
}
