use std::env;

use crate::{
    module::{
        Module, ModuleRef, module_error::ModuleError, module_kind::ModuleKind,
        module_loader::ModuleLoader,
    },
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

        if let Some(virtual_module) = self.check_if_virtual_module_and_load(module_path)? {
            return Ok(virtual_module);
        }

        if let Some(module) = self.get(&module_location_in_std) {
            return Ok(module.clone());
        }

        let module =
            Module::new(module_location_in_std.to_string(), ModuleKind::SourceFile)?.to_reference();

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

    pub fn check_if_virtual_module_and_load(
        &mut self,
        module_path: &str,
    ) -> Result<Option<ModuleRef>, ModuleError> {
        if !ModuleLoader::VIRTUAL_BUILTINS.contains(&module_path) {
            return Ok(None);
        }

        match module_path {
            "@std::math" => Ok(Some(self.try_load_math_module()?)),
            "@std::random" => Ok(Some(self.try_load_random_module()?)),
            "@std::_os" => Ok(Some(self.try_load_underscore_os_module()?)),
            "@std::_sys" => Ok(Some(self.try_load_underscore_sys_module()?)),
            "@std::_time" => Ok(Some(self.try_load_underscore_time_module()?)),
            "@std::_async" => Ok(Some(self.try_load_underscore_async_module()?)),
            "@std::_memory" => Ok(Some(self.try_load_underscore_memory_module()?)),
            "@std::_ntv" => Ok(Some(self.try_load_underscore_ntv_module()?)),
            "@std::_io" => Ok(Some(self.try_load_underscore_io_module()?)),
            _ => unreachable!(),
        }
    }
}
