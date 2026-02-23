use crate::{
    ast::expression::if_expression::IfExpression,
    object::{Object, null::Null, stack_environment::StackEnvironment},
};

impl IfExpression {
    pub fn evaluate(&self, environ: &mut StackEnvironment) -> Result<Object, String> {
        let condition = self.condition.evaluate(environ)?;

        if condition.is_truthy() {
            return self.consequence.evaluate(environ);
        } else if let Some(alternative) = &self.alternative {
            return alternative.evaluate(environ);
        }

        Ok(Object::Null(Null {}))
    }
}
