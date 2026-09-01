use std::path::PathBuf;

use crate::{
    module::{Module, ModuleRef, module_kind::ModuleKind},
    object::{
        Object::{self},
        built_in::BuiltIn,
        float::Float,
        new_objectref,
        stack_environment::StackEnvironment,
    },
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
        ("ceil", Object::BuiltIn(BuiltIn::MATH_CEIL)),
        ("floor", Object::BuiltIn(BuiltIn::MATH_FLOOR)),
        ("trunc", Object::BuiltIn(BuiltIn::MATH_TRUNC)),
        ("fabs", Object::BuiltIn(BuiltIn::MATH_FABS)),
        ("sqrt", Object::BuiltIn(BuiltIn::MATH_SQRT)),
        ("exp", Object::BuiltIn(BuiltIn::MATH_EXP)),
        ("expm1", Object::BuiltIn(BuiltIn::MATH_EXPM1)),
        ("log", Object::BuiltIn(BuiltIn::MATH_LOG)),
        ("log10", Object::BuiltIn(BuiltIn::MATH_LOG10)),
        ("log1p", Object::BuiltIn(BuiltIn::MATH_LOG1P)),
        ("pow", Object::BuiltIn(BuiltIn::MATH_POW)),
        ("fmod", Object::BuiltIn(BuiltIn::MATH_FMOD)),
        ("hypot", Object::BuiltIn(BuiltIn::MATH_HYPOT)),
        ("copysign", Object::BuiltIn(BuiltIn::MATH_COPYSIGN)),
        ("degrees", Object::BuiltIn(BuiltIn::MATH_DEGREES)),
        ("radians", Object::BuiltIn(BuiltIn::MATH_RADIANS)),
        ("sin", Object::BuiltIn(BuiltIn::MATH_SIN)),
        ("cos", Object::BuiltIn(BuiltIn::MATH_COS)),
        ("tan", Object::BuiltIn(BuiltIn::MATH_TAN)),
        ("asin", Object::BuiltIn(BuiltIn::MATH_ASIN)),
        ("acos", Object::BuiltIn(BuiltIn::MATH_ACOS)),
        ("atan", Object::BuiltIn(BuiltIn::MATH_ATAN)),
        ("atan2", Object::BuiltIn(BuiltIn::MATH_ATAN2)),
        ("sinh", Object::BuiltIn(BuiltIn::MATH_SINH)),
        ("cosh", Object::BuiltIn(BuiltIn::MATH_COSH)),
        ("tanh", Object::BuiltIn(BuiltIn::MATH_TANH)),
        ("asinh", Object::BuiltIn(BuiltIn::MATH_ASINH)),
        ("acosh", Object::BuiltIn(BuiltIn::MATH_ACOSH)),
        ("atanh", Object::BuiltIn(BuiltIn::MATH_ATANH)),
        ("ldexp", Object::BuiltIn(BuiltIn::MATH_LDEXP)),
        ("frexp", Object::BuiltIn(BuiltIn::MATH_FREXP)),
        ("modf", Object::BuiltIn(BuiltIn::MATH_MODF)),
        ("gcd", Object::BuiltIn(BuiltIn::MATH_GCD)),
        ("lcm", Object::BuiltIn(BuiltIn::MATH_LCM)),
        ("factorial", Object::BuiltIn(BuiltIn::MATH_FACTORIAL)),
        (
            "pi",
            Object::Float(Float {
                val: std::f64::consts::PI,
            }),
        ),
        (
            "e",
            Object::Float(Float {
                val: std::f64::consts::E,
            }),
        ),
        (
            "tau",
            Object::Float(Float {
                val: std::f64::consts::TAU,
            }),
        ),
        (
            "phi",
            Object::Float(Float {
                val: std::f64::consts::GOLDEN_RATIO,
            }),
        ),
    ];
    for (name, object) in pairs {
        let builtin_object = new_objectref(object);
        environ.insert_with_val_binding(name.into(), builtin_object);
    }
}
