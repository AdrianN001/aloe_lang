use std::panic;

use crate::{
    ast::{
        expression::{Expression, async_function_expression::AsyncFunctionExpression},
        statement::{Statement, async_function_statement::AsyncFunctionStatement},
    },
    object::{
        Object, ObjectRef, async_function::AsyncFunction, new_objectref, panic_obj::PanicObj,
        stack_environment::EnvRef,
    },
};

impl AsyncFunctionStatement {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, PanicObj> {
        match &*self.function {
            Statement::Function(function_stmt) => {
                let name = &function_stmt.name;

                let obj = new_objectref(Object::AsyncFunc(AsyncFunction {
                    parameters: function_stmt.parameters.clone(),
                    body: function_stmt.block.clone(),
                    env: environ.clone(),
                }));

                environ.borrow_mut().set(name, obj);

                return Ok(new_objectref(Object::NULL_OBJECT));
            }
            _ => panic!(),
        }
    }
}

impl AsyncFunctionExpression {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, PanicObj> {
        match &*self.function {
            Expression::Function(function) => {
                return Ok(new_objectref(Object::AsyncFunc(AsyncFunction {
                    parameters: function.parameters.clone(),
                    body: function.block.clone(),
                    env: environ.clone(),
                })));
            }
            _ => panic!(),
        }
    }
}
