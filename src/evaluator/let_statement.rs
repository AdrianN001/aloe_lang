use crate::{
    ast::{expression::Expression, statement::let_statement::LetStatement},
    object::{
        Object, ObjectRef, new_objectref, panic_obj::RuntimeSignal, return_value::ReturnValue,
        stack_environment::EnvRef, state::StateRef,
    },
};

impl LetStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }
        let value_assign_expr = match &self.assignment {
            Expression::ValueAssign(value_assign) => value_assign,
            _ => unreachable!(),
        };

        let value_res = value_assign_expr.evaluate_with_let_binding(environ.clone(), state.clone());

        let value = match value_res {
            Ok(ok_value) => ok_value,
            Err(RuntimeSignal::Yield(_)) => unreachable!(),
            Err(RuntimeSignal::Panic(_)) => return value_res,
            Err(RuntimeSignal::Propagation(err_obj)) => {
                return LetStatement::create_a_return_obj_from_err(err_obj);
            }
        };

        Ok(value.clone())
    }

    fn create_a_return_obj_from_err(err_obj: ObjectRef) -> Result<ObjectRef, RuntimeSignal> {
        let ret_oj = new_objectref(Object::ReturnVal(ReturnValue {
            value: Box::new(err_obj.clone()),
        }));
        Ok(ret_oj)
    }
}
