use crate::{
    ast::{expression::Expression, statement::val_statement::ValStatement},
    object::{ObjectRef, panic_obj::RuntimeSignal, stack_environment::EnvRef, state::StateRef},
};

impl ValStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }
        let value_assign_expr = match &self.assignment {
            Expression::ValueAssign(value_assign) => value_assign,
            _ => unreachable!(),
        };

        let value_res = value_assign_expr.evaluate_with_val_binding(environ.clone(), state.clone());

        let value = match value_res {
            Ok(ok_value) => ok_value,
            Err(RuntimeSignal::Yield(_)) => unreachable!(),
            Err(RuntimeSignal::Propagation(err_obj)) => {
                return Err(RuntimeSignal::Return(err_obj.clone()));
            }

            other => return other,
        };

        Ok(value.clone())
    }
}
