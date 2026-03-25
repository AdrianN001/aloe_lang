use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::identifier::Identifier,
    object::{
        Object, ObjectRef, built_in::BuiltIn, error::panic_type::PanicType, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef
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
            "__input" => Some(BuiltIn::Read),

            "type" => Some(BuiltIn::Type),
            "inspect" => Some(BuiltIn::Inspect),

            "range" => Some(BuiltIn::Range),
            "__random" => Some(BuiltIn::Random),

            "__err" => Some(BuiltIn::Err),
            _ => None,
        }
    }

    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match environ.borrow().get(&self.value) {
            Some(obj) => Ok(obj.clone()),
            None => {
                if let Some(built_in) = self.get_builtin_from_identifier() {
                    return Ok(Rc::new(RefCell::new(Object::BuiltIn(built_in))));
                }

                Err(PanicObj::new(
                    PanicType::UnknownIdentifier,
                    format!("unknown identifier: {}", &self.value),
                    state.clone(),
                ))
            }
        }
    }
}
