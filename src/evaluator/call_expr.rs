use crate::ast::expression::call_expression::CallExpression;
use crate::object::ObjectRef;
use crate::object::stack_environment::EnvRef;

impl CallExpression {
    pub fn evaluate_arguments(&self, environ: EnvRef) -> Result<Vec<ObjectRef>, String> {
        self.arguments
            .iter()
            .map(|argument| argument.evaluate(environ.clone()).clone())
            .collect()
    }
}
