use crate::{
    ast::expression::{Expression, value_assign_expression::ValueAssignExpression},
    object::{
        ObjectRef,
        error::panic_type::PanicType,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl ValueAssignExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let right = self.right.evaluate(environ.clone(), state.clone())?;

        match &*self.left {
            Expression::Identifier(identifier) => {
                let mut environ_borrow = environ.borrow_mut();
                if !environ_borrow.try_to_assign(&identifier.value, right.clone()) {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::VariableIsNotDeclared,
                        format!("variable '{}' is not initialized.", &identifier.value),
                        state.clone(),
                    )));
                }
                Ok(right.clone())
            }
            Expression::Index(index_expr) => {
                index_expr.evaluate_value_assign(environ.clone(), right.clone(), state.clone())?;
                Ok(right.clone())
            }
            Expression::Member(member_expr) => {
                member_expr.evaluate_value_assign(environ.clone(), state.clone(), right.clone())
            }
            other_expression => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::UnexpectedRValue,
                format!("expected LValue, got {}", other_expression.to_string()),
                state.clone(),
            ))),
        }
    }
}
