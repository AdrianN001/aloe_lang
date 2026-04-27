use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::array_literal::ArrayLiteral,
    object::{
        Object, ObjectRef, array::Array, panic_obj::RuntimeSignal, stack_environment::EnvRef,
        state::StateRef,
    },
};

impl ArrayLiteral {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let mut objects = Vec::new();

        for element in &self.elements {
            let obj = element.evaluate(environ.clone(), state.clone())?;
            if let Object::ReturnVal(_) = &*obj.borrow() {
                return Ok(obj.clone());
            }
            objects.push(obj);
        }

        Ok(Rc::new(RefCell::new(Object::Array(Array {
            items: objects,
        }))))
    }
}
