use crate::{
    ast::expression::{Expression, value_assign_expression::ValueAssignExpression},
    frame::{
        expr_frame::{EvaluationResult, ExpressionFrame},
        state::value_assign_state::ValueAssignState,
    },
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
                        format!("variable '{}' was not declared.", &identifier.value),
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

    pub fn eval_step(
        left_expr: &Expression,
        right_value: ObjectRef,
        environ: EnvRef,
        interpreter_state: StateRef,
        state: &ValueAssignState,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        match left_expr {
            Expression::Identifier(identifier) => {
                let mut environ_borrow = environ.borrow_mut();
                if !environ_borrow.try_to_assign(&identifier.value, right_value.clone()) {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::VariableIsNotDeclared,
                        format!("variable '{}' was not declared.", &identifier.value),
                        interpreter_state.clone(),
                    )));
                }
                Ok(EvaluationResult::Done(right_value.clone()))
            }
            Expression::Index(_) | Expression::Member(_) => {
                if state.left_value.is_none() {
                    return Ok(ExpressionFrame::build_frame_from_expr(left_expr, environ));
                }
                let left_value = state.left_value.as_ref().expect("already initialized");

                if left_value.as_ptr() != right_value.as_ptr() {
                    *left_value.borrow_mut() = right_value.borrow().clone();
                }
                Ok(EvaluationResult::Done(right_value.clone()))
            }
            other_expression => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::UnexpectedRValue,
                format!("expected LValue, got {}", other_expression.to_string()),
                interpreter_state.clone(),
            ))),
        }
    }
}
