use std::{cell::RefCell, fs, rc::Rc};

use crate::{
    module::{Module, module_error::ModuleError, module_loader::ModuleLoader},
    object::panic_obj::RuntimeSignal,
};

fn read_source_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

pub fn run_script(file_path: &str) -> Result<(), ModuleError> {
    let main_module = match Module::new(file_path.to_string()) {
        Ok(ok_value) => Rc::new(RefCell::new(ok_value)),
        Err(err) => return Err(err),
    };

    let mut module_cache = ModuleLoader::new(file_path);
    module_cache.set(main_module.clone());

    let result_of_the_script = main_module.borrow_mut().execute(&mut module_cache);

    if let Err(RuntimeSignal::Panic(panic_obj)) = result_of_the_script {
        println!("{}", panic_obj);
    }

    Ok(())
}
