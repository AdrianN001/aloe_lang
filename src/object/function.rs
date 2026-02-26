use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::expression::function_expression::FunctionExpression;
use crate::object::ObjectRef;
use crate::object::stack_environment::EnvRef;
use crate::{
    ast::{expression::identifier::Identifier, statement::block_statement::BlockStatement},
    object::{Object, stack_environment::StackEnvironment},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: EnvRef,
}

impl Function {
    pub fn get_type(&self) -> String {
        "function".into()
    }

    pub fn inspect(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("fn");
        buffer.push('(');

        buffer.push_str(
            &self
                .parameters
                .iter()
                .map(|parameter| parameter.value.clone())
                .collect::<Vec<_>>()
                .join(", "),
        );

        buffer.push_str(") {\n");
        buffer.push_str(&self.body.to_string());
        buffer.push_str("\n}");

        buffer
    }

    pub fn from_function_expression(expr: &FunctionExpression, environ: EnvRef) -> Self {
        Self {
            parameters: expr.parameters.clone(),
            body: expr.block.clone(),
            env: environ.clone(),
        }
    }

    // Function calling

    pub fn apply(&self, arguments: &[ObjectRef]) -> Result<ObjectRef, String> {
        let env = self.extend_environment_with_args(arguments);

        let last_expr = self.body.evaluate(env)?;
        let content_of_last_expr = last_expr.borrow();

        match &*content_of_last_expr {
            Object::ReturnVal(ret_val) => Ok(ret_val.unwrap_to_value()),
            _ => Ok(last_expr.clone()),
        }
    }

    fn extend_environment_with_args(&self, args: &[ObjectRef]) -> EnvRef {
        let mut new_env = StackEnvironment::new_enclosed(self.env.clone());

        self.parameters
            .iter()
            .enumerate()
            .for_each(|(indx, parameter)| {
                new_env.set(&parameter.value, args[indx].clone());
            });

        Rc::new(RefCell::new(new_env))
    }
}
