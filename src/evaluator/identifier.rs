use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::identifier::Identifier,
    object::{
        Object, ObjectRef,
        built_in::BuiltIn,
        error::panic_type::PanicType,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl Identifier {
    fn get_builtin_from_identifier(&self) -> Option<BuiltIn> {
        match self.value.as_str() {
            "len" => Some(BuiltIn::Len),

            "rest" => Some(BuiltIn::Rest),
            "first" => Some(BuiltIn::First),
            "last" => Some(BuiltIn::Last),
            "push" => Some(BuiltIn::Push),

            "print" => Some(BuiltIn::Print),
            "println" => Some(BuiltIn::Println),
            "input" => Some(BuiltIn::Read),
            "input_async" => Some(BuiltIn::ARead),

            "type" => Some(BuiltIn::Type),
            "inspect" => Some(BuiltIn::Inspect),
            "__line__" => Some(BuiltIn::Line),

            "range" => Some(BuiltIn::Range),
            "__random" => Some(BuiltIn::Random),

            "error" => Some(BuiltIn::Err),
            "panic" => Some(BuiltIn::Panic),

            "__exit" => Some(BuiltIn::Exit),
            "__args" => Some(BuiltIn::Args),
            "__pid" => Some(BuiltIn::Pid),

            "__sleep" => Some(BuiltIn::Sleep),
            "__sleep2" => Some(BuiltIn::Sleep2),
            "__time" => Some(BuiltIn::Time),

            "__spawn" => Some(BuiltIn::Spawn),

            "id" => Some(BuiltIn::Id),
            "__ref_n" => Some(BuiltIn::RefNumber),
            "__sizeof" => Some(BuiltIn::Size),

            "assert" => Some(BuiltIn::Assert),

            "__os_getenv" => Some(BuiltIn::GetEnv),
            "__os_setenv" => Some(BuiltIn::SetEnv),
            "__os_unsetenv" => Some(BuiltIn::UnsetEnv),
            "__os_get_current_dir" => Some(BuiltIn::CurrDir),
            "__os_get_home_dir" => Some(BuiltIn::HomeDir),
            "__os_get_temp_dir" => Some(BuiltIn::TempDir),
            "__os_platform" => Some(BuiltIn::Platform),
            "__os_arch" => Some(BuiltIn::Arch),

            "__ntv" => Some(BuiltIn::SpawnNative),
            _ => None,
        }
    }

    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        match environ.borrow().get(&self.value) {
            Some(obj) => Ok(obj.clone()),
            None => {
                if let Some(built_in) = self.get_builtin_from_identifier() {
                    return Ok(Rc::new(RefCell::new(Object::BuiltIn(built_in))));
                }

                Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::UnknownIdentifier,
                    format!("unknown identifier: {}", &self.value),
                    state.clone(),
                )))
            }
        }
    }
}
