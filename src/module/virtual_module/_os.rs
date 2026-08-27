use std::path::PathBuf;

use crate::{
    module::{Module, ModuleRef, module_kind::ModuleKind},
    object::{Object, built_in::BuiltIn, new_objectref, stack_environment::StackEnvironment},
};

pub fn create_underscore_os_module() -> ModuleRef {
    let mut environ = StackEnvironment::new();
    load_underscore_os_builtins(&mut environ);

    let env = environ.to_ref();

    let module = Module {
        environ: Some(env),
        kind: ModuleKind::Virtual,
        rel_path: PathBuf::with_capacity(0),
        abs_path: PathBuf::with_capacity(0),
        name: "_os".into(),
    };
    module.to_reference()
}

fn load_underscore_os_builtins(environ: &mut StackEnvironment) {
    let pairs = [
        ("__getenv", BuiltIn::U_OS_GET_ENV),
        ("__setenv", BuiltIn::U_OS_SET_ENV),
        ("__unsetenv", BuiltIn::U_OS_UNSET_ENV),
        ("__get_current_dir", BuiltIn::U_OS_CURRENT_DIR),
        ("__get_home_dir", BuiltIn::U_OS_HOME_DIR),
        ("__get_temp_dir", BuiltIn::U_OS_TEMP_DIR),
        ("__platform", BuiltIn::U_OS_PLATFORM),
        ("__arch", BuiltIn::U_OS_ARCH),
    ];
    for (name, builtin) in pairs.iter() {
        environ.insert_with_val_binding(name, new_objectref(Object::BuiltIn(builtin.clone())));
    }
}
