use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::statement::function_statement::FunctionStatement,
    object::{Object, ObjectRef, function::Function, new_objectref, stack_environment::EnvRef},
};

impl FunctionStatement {
    pub fn evaluate(&self, environ: EnvRef) -> ObjectRef {
        environ.borrow_mut().set_to_lowest_level(&self.name, {
            Rc::new(RefCell::new(Object::Func(Function {
                parameters: self.parameters.clone(),
                body: self.block.clone(),
                env: environ.clone(),
            })))
        });
        Rc::new(RefCell::new(Object::NULL_OBJECT))
    }

    pub fn evauluate_without_registering(&self, environ: EnvRef) -> ObjectRef{
        new_objectref(Object::Func(Function { 
            parameters: self.parameters.clone(), 
            body: self.block.clone(), 
            env: environ.clone() 
        }))
    }
}
