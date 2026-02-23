use crate::ast::expression::call_expression::CallExpression;
use crate::object::Object;
use crate::object::stack_environment::StackEnvironment;

impl CallExpression {
    pub fn evaluate_arguments(
        &self,
        environ: &mut StackEnvironment,
    ) -> Result<Vec<Object>, String> {
        self.arguments
            .iter()
            .map(|argument| argument.evaluate(environ))
            .collect()
    }
}
