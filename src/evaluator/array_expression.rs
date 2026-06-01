use crate::{
    ast::expression::array_literal::ArrayLiteral,
    object::{
        Object, ObjectRef, array::Array, new_objectref, panic_obj::RuntimeSignal,
        stack_environment::EnvRef, state::StateRef,
    },
};

impl ArrayLiteral {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }
        let mut objects = Vec::new();

        for element in &self.elements {
            let obj = element.evaluate(environ.clone(), state.clone())?;
            if let Object::ReturnVal(_) = &*obj.borrow() {
                return Ok(obj.clone());
            }
            objects.push(obj);
        }

        Ok(new_objectref(Object::Array(Box::new(Array {
            items: objects,
        }))))
    }

    pub fn eval_step(objects: &[ObjectRef], _state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let mut mapped_objects = Vec::new();

        for object in objects {
            let is_propagated_error = {
                //TODO: kann sein das es eine Fehler zuruckgibt, wenn die Array sich beinhaltet.
                let obj_borrow = object.borrow();
                matches!(*obj_borrow, Object::ReturnVal(_))
            };

            if is_propagated_error {
                return Ok(object.clone());
            }

            mapped_objects.push(object.clone());
        }

        Ok(new_objectref(Object::Array(Box::new(Array {
            items: mapped_objects,
        }))))
    }
}
