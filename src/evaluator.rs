
mod prefix_expr;
mod infix_expr;

use crate::object::integer::Integer;

use super::object::Object;

use super::ast::expression::Expression;
use super::ast::statement::Statement;



impl Expression{
    pub fn evaluate(&self) -> Result<Object, String>{
        
        match self{
            Expression::IntegerLiteral(literal) => Ok(Object::Int(Integer{
                value: literal.value
            })),
            Expression::Bool(bool_literal) => Ok(if bool_literal.value { 
                Object::TRUE_BOOL_OBJECT 
            } else{ 
                Object::FALSE_BOOL_OBJECT 
            }),
            Expression::Prefix(prefix_expr) => {
                let right_side = prefix_expr.right.evaluate()?;
                right_side.evaluate_prefix(&prefix_expr.operator)
            },
            Expression::Infix(infix_expr) =>{
                let right_side = infix_expr.right.evaluate()?;
                let left_side =  infix_expr.left.evaluate()?;

                left_side.evaluate_infix_expression(&right_side, &infix_expr.operator)
            }
            _ => panic!()
        }
    }
}


impl Statement{

    pub fn evaluate(&self) -> Result<Object, String>{
        match self{
            Statement::Expression(expr) => expr.expression.evaluate(),
            _ => panic!()
        }
    }
}
