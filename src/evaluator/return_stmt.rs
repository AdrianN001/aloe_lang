use crate::{
    ast::statement::return_statement::ReturnStatement,
    object::{
        Object, ObjectRef, new_objectref, panic_obj::RuntimeSignal, return_value::ReturnValue,
        stack_environment::EnvRef, state::StateRef,
    },
};

impl ReturnStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }
        let val = match &self.value {
            Some(return_value) => {
                let ret_val_obj_res = return_value.evaluate(environ, state);
                let (_, value) = ReturnStatement::check_if_value_is_propagation(ret_val_obj_res)?;
                value
            }
            None => new_objectref(Object::NULL_OBJECT),
        };
        if let Object::ReturnVal(ret_val) = &*val.borrow() {
            return Ok(ret_val.unwrap_to_value().clone());
        }

        Ok(new_objectref(Object::ReturnVal(ReturnValue {
            value: Box::new(val.clone()),
        })))
    }

    pub fn check_if_value_is_propagation(
        value_res: Result<ObjectRef, RuntimeSignal>,
    ) -> Result<(bool, ObjectRef), RuntimeSignal> {
        match value_res {
            Ok(ok_value) => return Ok((false, ok_value)),
            Err(RuntimeSignal::Panic(panic_obj)) => return Err(RuntimeSignal::Panic(panic_obj)),
            Err(RuntimeSignal::Yield(_)) => unreachable!(),
            Err(RuntimeSignal::Break(val)) => return Err(RuntimeSignal::Break(val)),
            Err(RuntimeSignal::Continue) => return Err(RuntimeSignal::Continue),
            Err(RuntimeSignal::Propagation(progated_err)) => return Ok((true, progated_err)),
        }
    }
}
