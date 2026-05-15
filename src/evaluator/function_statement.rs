use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::statement::function_statement::FunctionStatement,
    object::{
        Object, ObjectRef,
        function::Function,
        new_objectref,
        stack_environment::{EnvRef, StackEnvironment},
    },
};

impl FunctionStatement {
    pub fn evaluate(&self, environ: EnvRef) -> ObjectRef {
        let function_object = new_objectref(Object::Func(Function {
            parameters: self.parameters.clone(),
            body: self.block.clone(),
            env: {
                let new_environemnt = StackEnvironment::new_enclosed(
                    environ.clone(),
                    format!("function {}(...) {{...}}", self.name),
                )
                .to_ref();

                {
                    let mut env_borrow = new_environemnt.borrow_mut();
                    env_borrow.set_loop_context(false);
                };

                new_environemnt
            },
        }));
        environ
            .borrow_mut()
            .set(&self.name, function_object.clone());
        Rc::new(RefCell::new(Object::NULL_OBJECT))
    }

    pub fn evauluate_without_registering(&self, environ: EnvRef) -> ObjectRef {
        new_objectref(Object::Func(Function {
            parameters: self.parameters.clone(),
            body: self.block.clone(),
            env: {
                let new_environemnt = StackEnvironment::new_enclosed(
                    environ.clone(),
                    format!("function {}(...) {{...}}", self.name),
                )
                .to_ref();

                {
                    let mut env_borrow = new_environemnt.borrow_mut();
                    env_borrow.set_loop_context(false);
                }
                new_environemnt
            },
        }))
    }
}
