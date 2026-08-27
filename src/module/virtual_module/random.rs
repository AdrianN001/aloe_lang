use std::path::PathBuf;

use crate::{
    module::{Module, ModuleRef, module_kind::ModuleKind},
    object::{Object, built_in::BuiltIn, new_objectref, stack_environment::StackEnvironment},
};

pub fn create_random_module() -> ModuleRef {
    let mut environ = StackEnvironment::new();
    load_random_builtins(&mut environ);

    let env = environ.to_ref();

    let module = Module {
        environ: Some(env),
        kind: ModuleKind::Virtual,
        rel_path: PathBuf::with_capacity(0),
        abs_path: PathBuf::with_capacity(0),
        name: "random".into(),
    };
    module.to_reference()
}

fn load_random_builtins(environ: &mut StackEnvironment) {
    let pairs = [("_rnd", BuiltIn::RANDOM_RND)];
    for (name, builtin) in pairs {
        let builtin_object = new_objectref(Object::BuiltIn(builtin));
        environ.insert_with_val_binding(name.into(), builtin_object);
    }
}
