use crate::{
    ast::expression::{Expression, call_expression::CallExpression, member::MemberExpression},
    object::{ObjectRef, stack_environment::EnvRef},
};

impl MemberExpression {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let left_obj = self.left.evaluate(environ.clone())?;

        match &*self.right {
            Expression::Call(call_expr) => {
                let name_of_method = Self::get_call_expressions_identifier(call_expr)?;
                let args = call_expr.evaluate_arguments(environ.clone())?;

                let mut obj = left_obj.borrow_mut();

                Ok(obj.apply_method(&name_of_method, &args, environ.clone()))
            }
            Expression::Identifier(identifier_expr) => {
                let name_of_attribute = &identifier_expr.value;

                let obj = left_obj.borrow();
                Ok(obj.apply_attribute(name_of_attribute))
            }
            _ => Err("illegal expression type".to_string()),
        }
    }

    fn get_call_expressions_identifier(call_expr: &CallExpression) -> Result<String, String> {
        match &*call_expr.function {
            Expression::Identifier(identifier) => Ok(identifier.value.clone()),
            _ => Err("illegal expression".into()),
        }
    }
}
