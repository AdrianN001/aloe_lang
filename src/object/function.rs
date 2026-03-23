use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::expression::function_expression::FunctionExpression;
use crate::object::ObjectRef;
use crate::object::panic_obj::PanicObj;
use crate::object::stack_environment::EnvRef;
use crate::object::state::StateRef;
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

    pub fn apply(
        &self,
        name_of_the_function: String,
        arguments: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        if arguments.len() != self.parameters.len() {
            return Err(PanicObj::new(
                format!(
                    "expected {} arguments, got: {}",
                    self.parameters.len(),
                    arguments.len()
                ),
                state.clone(),
            ));
        }

        {
            state
                .borrow_mut()
                .push_to_stack(name_of_the_function.clone());
        }

        let env = self.extend_environment_with_args(name_of_the_function, arguments);

        let last_expr = self
            .body
            .evaluate_with_function_context(env, state.clone())?;

        {
            state.borrow_mut().pop_from_stack();
        }

        let content_of_last_expr = last_expr.borrow();

        match &*content_of_last_expr {
            Object::ReturnVal(ret_val) => Ok(ret_val.unwrap_to_value()),
            _ => Ok(last_expr.clone()),
        }
    }

    fn extend_environment_with_args(
        &self,
        name_of_the_function: String,
        args: &[ObjectRef],
    ) -> EnvRef {
        let mut new_env =
            StackEnvironment::new_enclosed(self.env.clone(), format!("{}()", name_of_the_function));

        self.parameters
            .iter()
            .enumerate()
            .for_each(|(indx, parameter)| {
                new_env.set_to_lowest_level(&parameter.value, args[indx].clone());
            });

        Rc::new(RefCell::new(new_env))
    }
}
