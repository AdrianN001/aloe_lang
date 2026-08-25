use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{
    module::{
        Module, module_error::ModuleError, module_kind::ModuleKind, module_loader::ModuleLoader,
    },
    object::panic_obj::RuntimeSignal,
};

pub fn run_script(file_path: &PathBuf) -> Result<(), ModuleError> {
    let main_module = match Module::new(
        file_path.to_str().unwrap().to_string(),
        ModuleKind::SourceFile,
    ) {
        Ok(ok_value) => Rc::new(RefCell::new(ok_value)),
        Err(err) => return Err(err),
    };

    let mut module_cache = ModuleLoader::new(file_path);
    module_cache.set(main_module.clone());

    let result_of_the_script = Module::execute(main_module.clone(), &mut module_cache);

    if let Err(RuntimeSignal::Panic(panic_obj)) = result_of_the_script {
        println!("{}", panic_obj);
    }

    Ok(())
}

pub fn run_artifact(file_path: &PathBuf) -> Result<(), ModuleError> {
    let main_module = match Module::new(
        file_path.to_str().unwrap().to_string(),
        ModuleKind::ArtifactFile,
    ) {
        Ok(ok_value) => Rc::new(RefCell::new(ok_value)),
        Err(err) => return Err(err),
    };

    let mut module_cache = ModuleLoader::new(file_path);
    module_cache.set(main_module.clone());

    let result_of_the_script = Module::execute(main_module.clone(), &mut module_cache);

    if let Err(RuntimeSignal::Panic(panic_obj)) = result_of_the_script {
        println!("{}", panic_obj);
    }

    Ok(())
}
