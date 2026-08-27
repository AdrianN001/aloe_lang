use std::path::PathBuf;

use crate::{
    module::{Module, ModuleRef, module_kind::ModuleKind},
    object::{Object, built_in::BuiltIn, new_objectref, stack_environment::StackEnvironment},
};

pub fn create_math_module() -> ModuleRef {
    let mut environ = StackEnvironment::new();
    load_math_builtins(&mut environ);

    let env = environ.to_ref();

    let module = Module {
        environ: Some(env),
        kind: ModuleKind::Virtual,
        rel_path: PathBuf::with_capacity(0),
        abs_path: PathBuf::with_capacity(0),
        name: "math".into(),
    };
    module.to_reference()
}

fn load_math_builtins(environ: &mut StackEnvironment) {
    let pairs = [
        ("ceil", BuiltIn::Ceil),
        ("floor", BuiltIn::Floor),
        ("trunc", BuiltIn::Trunc),
        ("fabs", BuiltIn::Fabs),
        ("sqrt", BuiltIn::Sqrt),
        ("exp", BuiltIn::Exp),
        ("expm1", BuiltIn::Expm1),
        ("log", BuiltIn::Log),
        ("log10", BuiltIn::Log10),
        ("log1p", BuiltIn::Log1p),
        ("pow", BuiltIn::Pow),
        ("fmod", BuiltIn::Fmod),
        ("hypot", BuiltIn::Hypot),
        ("copysign", BuiltIn::Copysign),
        ("degrees", BuiltIn::Degrees),
        ("radians", BuiltIn::Radians),
        ("sin", BuiltIn::Sin),
        ("cos", BuiltIn::Cos),
        ("tan", BuiltIn::Tan),
        ("asin", BuiltIn::Asin),
        ("acos", BuiltIn::Acos),
        ("atan", BuiltIn::Atan),
        ("atan2", BuiltIn::Atan2),
        ("sinh", BuiltIn::Sinh),
        ("cosh", BuiltIn::Cosh),
        ("tanh", BuiltIn::Tanh),
        ("asinh", BuiltIn::Asinh),
        ("acosh", BuiltIn::Acosh),
        ("atanh", BuiltIn::Atanh),
        ("ldexp", BuiltIn::Ldexp),
        ("frexp", BuiltIn::Frexp),
        ("modf", BuiltIn::Modf),
        ("gcd", BuiltIn::Gcd),
        ("lcm", BuiltIn::Lcm),
        ("factorial", BuiltIn::Factorial),
    ];
    for (name, builtin) in pairs {
        let builtin_object = new_objectref(Object::BuiltIn(builtin));
        environ.insert_with_val_binding(name.into(), builtin_object);
    }
}
