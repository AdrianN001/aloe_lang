use crate::{
    ast::statement::return_statement::ReturnStatement,
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl ReturnStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }

        if !state.borrow().is_function_context() {
            return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                PanicType::ReturnFromNonfunctionalContext,
                "unexpected return keyword in non-function context",
                state.clone(),
            )));
        }

        let val = match &self.value {
            Some(return_value) => {
                let ret_val_obj_res = return_value.evaluate(environ, state);
                let (_, value) = ReturnStatement::check_if_value_is_propagation(ret_val_obj_res)?;
                value
            }
            None => new_objectref(Object::NULL_OBJECT),
        };

        Err(RuntimeSignal::Return(val.clone()))
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
            Err(RuntimeSignal::Return(val)) => return Err(RuntimeSignal::Return(val)),
        }
    }
}
