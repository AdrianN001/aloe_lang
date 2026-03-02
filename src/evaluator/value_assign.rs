use crate::{
    ast::expression::{Expression, value_assign_expression::ValueAssignExpression},
    object::{Object, ObjectRef, stack_environment::EnvRef},
};

impl ValueAssignExpression {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let right = self.right.evaluate(environ.clone())?;

        let right_assignable_value = match &*right.borrow() {
            Object::ReturnVal(ret) => *ret.value.clone(),
            _ => right.clone(),
        };

        match &*self.left {
            Expression::Identifier(identifier) => {
                let mut environ_borrow = environ.borrow_mut();
                if !environ_borrow.try_to_assign(&identifier.value, right_assignable_value.clone())
                {
                    return Err(format!(
                        "variable '{}' is not initialized.",
                        &identifier.value
                    ));
                }
                Ok(right_assignable_value.clone())
            }
            Expression::Index(index_expr) => {
                index_expr.evaluate_value_assign(environ.clone(), right_assignable_value)?;
                Ok(right.clone())
            }
            other_expression => Err(format!(
                "expected LValue, got {}",
                other_expression.to_string()
            )),
        }
    }
}
