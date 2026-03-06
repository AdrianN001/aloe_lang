use crate::ast::expression::Expression;
use crate::ast::expression::call_expression::CallExpression;
use crate::object::stack_environment::EnvRef;
use crate::object::state::StateRef;
use crate::object::{Object, ObjectRef};

impl CallExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, String> {
        let function_object = self
            .function
            .evaluate(environ.clone(), state.clone())?
            .clone();

        let function_name = match &*self.function {
            Expression::Identifier(identifier) => identifier.value.clone(),
            _ => "(anonymm function)".to_string(),
        };

        let args = self.evaluate_arguments(environ.clone(), state.clone())?;

        match &*function_object.borrow() {
            Object::Func(function) => function.apply(function_name, &args, state.clone()),
            Object::BuiltIn(built_in_function) => {
                Ok(built_in_function.call(&args, environ.clone(), state.clone()))
            }
            other_type => Err(format!(
                "'{}' is not a function. It cannot be called.",
                other_type.inspect()
            )),
        }
    }

    pub fn evaluate_arguments(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<Vec<ObjectRef>, String> {
        self.arguments
            .iter()
            .map(|argument| argument.evaluate(environ.clone(), state.clone()).clone())
            .collect()
    }
}
