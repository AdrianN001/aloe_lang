use crate::{ast::{expression::{Expression, value_assign_expression::ValueAssignExpression}}, object::{ObjectRef, stack_environment::EnvRef}};



impl ValueAssignExpression{

    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String>{
        let right = self.right.evaluate(environ.clone())?;

        match &*self.left{
            Expression::Identifier(identifier) => {
                let mut environ_borrow = environ.borrow_mut();
                if environ_borrow.get(&identifier.value).is_some(){
                    environ_borrow.set(&identifier.value, right.clone());
                }else{
                    return Err(format!("variable '{}' is not initialized.", &identifier.value));
                }
                Ok(right.clone())
            },
            Expression::Index(index_expr) => {
                index_expr.evaluate_value_assign(environ.clone(), right.clone())?;
                Ok(right.clone())
            }
            , 
            other_expression => Err(format!("expected LValue, got {}", other_expression.to_string()))
        }
    }
}
