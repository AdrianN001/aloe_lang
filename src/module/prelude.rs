use crate::{
    module::Module,
    object::{Object, built_in::BuiltIn, new_objectref, stack_environment::StackEnvironment},
};

impl Module {
    pub const PRELUDES: &[(&'static str, BuiltIn); 11] = &[
        ("print", BuiltIn::P_PRINT),
        ("println", BuiltIn::P_PRINTLN),
        ("input", BuiltIn::P_READ),
        ("len", BuiltIn::P_LEN),
        ("range", BuiltIn::P_RANGE),
        ("error", BuiltIn::P_ERROR),
        ("panic", BuiltIn::P_PANIC),
        ("type", BuiltIn::P_TYPE),
        ("inspect", BuiltIn::P_INSPECT),
        ("_line_", BuiltIn::P_LINE),
        ("assert", BuiltIn::P_ASSERT),
    ];

    pub fn load_prelude(environ: &mut StackEnvironment) {
        for (name, built_in) in Self::PRELUDES {
            let object = new_objectref(Object::BuiltIn(built_in.clone()));

            environ.insert_with_val_binding(name, object);
        }
    }
}
