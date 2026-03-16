use crate::{
    ast::expression::{Expression, value_assign_expression::ValueAssignExpression},
    object::{ObjectRef, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef},
};

impl ValueAssignExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let right = self.right.evaluate(environ.clone(), state.clone())?;

        match &*self.left {
            Expression::Identifier(identifier) => {
                let mut environ_borrow = environ.borrow_mut();
                if !environ_borrow.try_to_assign(&identifier.value, right.clone()) {
                    return Err(PanicObj::new(format!(
                        "variable '{}' is not initialized.",
                        &identifier.value
                    ), state.clone()));
                }
                Ok(right.clone())
            }
            Expression::Index(index_expr) => {
                index_expr.evaluate_value_assign(environ.clone(), right.clone(), state.clone())?;
                Ok(right.clone())
            }
            other_expression => Err(PanicObj::new(format!(
                "expected LValue, got {}",
                other_expression.to_string()
            ), state.clone())),
        }
    }
}
