use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{
    module::{Module, module_kind::ModuleKind, module_loader::ModuleLoader},
    object::panic_obj::RuntimeSignal,
};

pub fn run_script(file_path: &PathBuf) -> Result<(), RuntimeSignal> {
    let main_module = match Module::new(file_path.display().to_string(), ModuleKind::SourceFile) {
        Ok(ok_value) => Rc::new(RefCell::new(ok_value)),
        Err(err) => return Err(RuntimeSignal::ModuleLoadError(err)),
    };

    let mut module_cache = ModuleLoader::new(file_path);
    module_cache.set(main_module.clone());

    Module::execute(main_module.clone(), &mut module_cache)
}

pub fn run_artifact(file_path: &PathBuf) -> Result<(), RuntimeSignal> {
    let main_module = match Module::new(file_path.display().to_string(), ModuleKind::ArtifactFile) {
        Ok(ok_value) => Rc::new(RefCell::new(ok_value)),
        Err(err) => return Err(RuntimeSignal::ModuleLoadError(err)),
    };

    let mut module_cache = ModuleLoader::new(file_path);
    module_cache.set(main_module.clone());

    Module::execute(main_module.clone(), &mut module_cache)
}
