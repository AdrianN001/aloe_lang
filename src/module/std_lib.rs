use std::env;

use crate::{
    module::{Module, ModuleRef, module_error::ModuleError, module_loader::ModuleLoader},
    object::panic_obj::RuntimeSignal,
    version::CURRENT_VERSION,
};

pub const STANDARD_LIBRARY_IDENTIFIER: &'static str = "@std::";

//TODO: check the os, on windows /usr/lib/aloe/std/ is an invalid path
pub fn get_std_lib_location(module_path: &str, version_str: &str) -> Result<String, ModuleError> {
    let home_dir = match env::home_dir() {
        None => {
            return Err(ModuleError::new(
                module_path,
                "could not get your home directory (~)",
            ));
        }
        Some(home_dir_path) => home_dir_path.display().to_string(),
    };
    Ok(format!("{}/.aloe/std/{}/", home_dir, version_str))
}

impl ModuleLoader {
    fn transfrom_std_path_to_abs(module_path: &str) -> Result<String, ModuleError> {
        let current_version_as_str = CURRENT_VERSION.to_string();
        let std_lib_location = get_std_lib_location(module_path, &current_version_as_str)?;

        Ok(module_path.replacen(STANDARD_LIBRARY_IDENTIFIER, &std_lib_location, 1))
    }
    pub fn import_from_std(&mut self, module_path: &str) -> Result<ModuleRef, ModuleError> {
        let module_location_in_std = ModuleLoader::transfrom_std_path_to_abs(module_path)?;

        if let Some(module) = self.get(&module_location_in_std) {
            return Ok(module.clone());
        }

        let module = Module::new(module_location_in_std.to_string())?.to_reference();

        self.set(module.clone());

        match Module::execute(module.clone(), self) {
            Ok(_) => {}
            Err(RuntimeSignal::Panic(e)) => {
                return Err(ModuleError::new(
                    module_path,
                    &format!("module execution failed: \n{}", e),
                ));
            }
            _ => unreachable!(),
        }

        Ok(module)
    }
}
