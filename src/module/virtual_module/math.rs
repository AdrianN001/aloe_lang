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
        ("ceil", BuiltIn::MATH_CEIL),
        ("floor", BuiltIn::MATH_FLOOR),
        ("trunc", BuiltIn::MATH_TRUNC),
        ("fabs", BuiltIn::MATH_FABS),
        ("sqrt", BuiltIn::MATH_SQRT),
        ("exp", BuiltIn::MATH_EXP),
        ("expm1", BuiltIn::MATH_EXPM1),
        ("log", BuiltIn::MATH_LOG),
        ("log10", BuiltIn::MATH_LOG10),
        ("log1p", BuiltIn::MATH_LOG1P),
        ("pow", BuiltIn::MATH_POW),
        ("fmod", BuiltIn::MATH_FMOD),
        ("hypot", BuiltIn::MATH_HYPOT),
        ("copysign", BuiltIn::MATH_COPYSIGN),
        ("degrees", BuiltIn::MATH_DEGREES),
        ("radians", BuiltIn::MATH_RADIANS),
        ("sin", BuiltIn::MATH_SIN),
        ("cos", BuiltIn::MATH_COS),
        ("tan", BuiltIn::MATH_TAN),
        ("asin", BuiltIn::MATH_ASIN),
        ("acos", BuiltIn::MATH_ACOS),
        ("atan", BuiltIn::MATH_ATAN),
        ("atan2", BuiltIn::MATH_ATAN2),
        ("sinh", BuiltIn::MATH_SINH),
        ("cosh", BuiltIn::MATH_COSH),
        ("tanh", BuiltIn::MATH_TANH),
        ("asinh", BuiltIn::MATH_ASINH),
        ("acosh", BuiltIn::MATH_ACOSH),
        ("atanh", BuiltIn::MATH_ATANH),
        ("ldexp", BuiltIn::MATH_LDEXP),
        ("frexp", BuiltIn::MATH_FREXP),
        ("modf", BuiltIn::MATH_MODF),
        ("gcd", BuiltIn::MATH_GCD),
        ("lcm", BuiltIn::MATH_LCM),
        ("factorial", BuiltIn::MATH_FACTORIAL),
    ];
    for (name, builtin) in pairs {
        let builtin_object = new_objectref(Object::BuiltIn(builtin));
        environ.insert_with_val_binding(name.into(), builtin_object);
    }
}
