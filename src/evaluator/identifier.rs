use crate::{ast::expression::identifier::Identifier, object::{Object, stack_environment::StackEnvironment}};



impl Identifier{

    pub fn evaluate(&self, environ: &StackEnvironment) -> Result<Object, String>{
        match environ.get_owned(&self.value){
            Some(obj) => Ok(obj),
            None => Err("unknown identifier".into())
        }
    } 
}
