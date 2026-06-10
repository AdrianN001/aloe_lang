use std::panic;

use crate::{
    ast::{
        expression::{Expression, async_function_expression::AsyncFunctionExpression},
        statement::{Statement, async_function_statement::AsyncFunctionStatement},
    },
    object::{
        Object, ObjectRef,
        async_function::AsyncFunction,
        new_objectref,
        panic_obj::RuntimeSignal,
        stack_environment::{EnvRef, StackEnvironment},
    },
};

impl AsyncFunctionStatement {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, RuntimeSignal> {
        match &*self.function {
            Statement::Function(function_stmt) => {
                let name = &function_stmt.name;

                let new_environment = StackEnvironment::new_enclosed(
                    environ.clone(),
                    format!("async fun {}(...) {{...}}", name),
                )
                .to_ref();

                {
                    let mut env_borrow = new_environment.borrow_mut();
                    env_borrow.set_loop_context(false);
                }

                let obj = new_objectref(Object::AsyncFunc(Box::new(AsyncFunction {
                    parameters: function_stmt.parameters.clone(),
                    body: function_stmt.block.clone(),
                    env: new_environment,
                })));

                environ.borrow_mut().insert_with_let_binding(name, obj);

                Ok(new_objectref(Object::NULL_OBJECT))
            }
            _ => panic!(),
        }
    }

    pub fn evaluate_without_registering(
        &self,
        environ: EnvRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        match &*self.function {
            Statement::Function(function_stmt) => {
                let new_environment = StackEnvironment::new_enclosed(
                    environ.clone(),
                    format!("async fun {}(...) {{...}}", function_stmt.name),
                )
                .to_ref();

                {
                    let mut env_borrow = new_environment.borrow_mut();
                    env_borrow.set_loop_context(false);
                }
                let obj = new_objectref(Object::AsyncFunc(Box::new(AsyncFunction {
                    parameters: function_stmt.parameters.clone(),
                    body: function_stmt.block.clone(),
                    env: new_environment,
                })));

                Ok(obj)
            }
            _ => panic!(),
        }
    }
}

impl AsyncFunctionExpression {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, RuntimeSignal> {
        match &*self.function {
            Expression::Function(function) => {
                let new_environment = StackEnvironment::new_enclosed(
                    environ.clone(),
                    format!("async fn(...) {{...}}"),
                )
                .to_ref();

                {
                    let mut env_borrow = new_environment.borrow_mut();
                    env_borrow.set_loop_context(false);
                }

                Ok(new_objectref(Object::AsyncFunc(Box::new(AsyncFunction {
                    parameters: function.parameters.clone(),
                    body: function.block.clone(),
                    env: new_environment,
                }))))
            }
            //TODO: launch (async fn(){})() panics
            _ => panic!(),
        }
    }
}
