use crate::{
    module::Module,
    object::{Object, built_in::BuiltIn, new_objectref, stack_environment::StackEnvironment},
};

impl Module {
    pub const PRELUDES: &[(&'static str, BuiltIn); 11] = &[
        ("print", BuiltIn::Print),
        ("println", BuiltIn::Println),
        ("input", BuiltIn::Read),
        ("len", BuiltIn::Len),
        ("range", BuiltIn::Range),
        ("error", BuiltIn::Error),
        ("panic", BuiltIn::Panic),
        ("type", BuiltIn::Type),
        ("inspect", BuiltIn::Inspect),
        ("_line_", BuiltIn::Line),
        ("assert", BuiltIn::Assert),
    ];

    pub fn load_prelude(environ: &mut StackEnvironment) {
        for (name, built_in) in Self::PRELUDES {
            let object = new_objectref(Object::BuiltIn(built_in.clone()));

            environ.insert_with_val_binding(name, object);
        }
    }
}
