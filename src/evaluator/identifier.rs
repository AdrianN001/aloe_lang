use crate::{
    ast::expression::identifier::Identifier,
    object::{
        ObjectRef,
        error::panic_type::PanicType,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl Identifier {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        match environ.borrow().get(&self.value) {
            Some(obj) => Ok(obj.clone()),
            None => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::UnknownIdentifier,
                format!("unknown identifier: {}", &self.value),
                state.clone(),
            ))),
        }
    }
}
