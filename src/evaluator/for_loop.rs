use std::{cell::RefCell, rc::Rc};

use crate::{ast::expression::{Expression, for_loop::ForLoopExpression, identifier::Identifier}, object::{Object, ObjectRef, stack_environment::{EnvRef, StackEnvironment}}};





impl ForLoopExpression{
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let new_environment = Rc::new(RefCell::new(
                StackEnvironment::new_enclosed(environ.clone())
        ));

        if let Some(variable) = &self.variable && let Some(iteratable) = &self.iterator{
            return match (&**variable, &**iteratable){
                (Expression::Identifier(identifier), iterable_expression) => self.evaluate_normal_for_loop(new_environment, identifier, iterable_expression),
                _ => return Err("err".into())
            };
        }


        Err("err".to_string())
    }

    fn evaluate_normal_for_loop(&self, environ: EnvRef, variable: &Identifier, iterable: &Expression) -> Result<ObjectRef, String>{
       
        let provided_object = iterable.evaluate(environ.clone())?;
        let mut iterator = match &*provided_object.borrow(){
            Object::Iterator(iterator) => iterator.clone(),
            _ => return Err("value provided is not an iterator".into()),
        };

        while let Some(current_value) = iterator._next(){    
            environ.borrow_mut().set(
                &variable.value, current_value
            );

            for statement in &self.block.statements{
                let result = statement.evaluate(environ.clone())?;

                if let Object::ReturnVal(_) = &*result.borrow(){
                    return Ok(result.clone());
                } else if let Object::BreakVal(break_val) = &*result.borrow(){
                    return Ok(*break_val.value.clone())
                } else if matches!(&*result.borrow(), Object::Continue){
                    break;
                }
            }
        }

        Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)))
    }
}
