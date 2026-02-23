
mod index_expr;
mod prefix_expr;
mod infix_expr;
mod if_expression;
mod identifier;
mod call_expr;
mod block_statement;
mod hash_literal;

use crate::ast::program::Program;
use crate::object::array::Array;
use crate::object::function::Function;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::return_value::ReturnValue;
use crate::object::stack_environment::StackEnvironment;
use crate::object::string_obj::StringObj;


use super::object::Object;

use super::ast::expression::Expression;
use super::ast::statement::Statement;



impl Expression{
    pub fn evaluate(&self, environ: &mut StackEnvironment) -> Result<Object, String>{
        
        match self{
            Expression::IntegerLiteral(literal) => Ok(Object::Int(Integer{
                value: literal.value
            })),
            Expression::Identifier(identifier) => identifier.evaluate(environ), 
            Expression::Bool(bool_literal) => Ok(Object::get_native_boolean_object(bool_literal.value)),
            Expression::Prefix(prefix_expr) => {
                let right_side = prefix_expr.right.evaluate(environ)?;
                right_side.evaluate_prefix(&prefix_expr.operator)
            },
            Expression::HashMapLiteral(hashmap) => hashmap.evaluate(environ),
            Expression::Index(indx_expr) => indx_expr.evaluate(environ),
            Expression::String(str_exr) => Ok(Object::String(StringObj{
                value: str_exr.value.clone() 
            })),
            Expression::Function(func_expr) => {
                Ok(Object::Func(Function::from_function_expression(func_expr, environ)))
            },
            Expression::Call(call_expr) => {
                let function_object = call_expr.function.evaluate(environ)?;
                
                let args = call_expr.evaluate_arguments(environ)?;

                match function_object{
                    Object::Func(function) => function.apply(&args),
                    Object::BuiltIn(built_in_function) => Ok(built_in_function.call(&args)),
                    _ => Err("not a function".into())
                }
            },
            Expression::Array(array) => {
                let mut objects = Vec::new();

                for element in &array.elements{
                    objects.push(
                        element.evaluate(environ)?
                    );
                }

                Ok(Object::Array(Array{
                    items: objects
                }))
            },
            Expression::If(if_expression) => if_expression.evaluate(environ),
            Expression::Infix(infix_expr) =>{
                let right_side = infix_expr.right.evaluate(environ)?;
                let left_side =  infix_expr.left.evaluate(environ)?;

                left_side.evaluate_infix_expression(&right_side, &infix_expr.operator)
            },
            _ => panic!("unexpected expression type")
        }
    }
}


impl Statement{

    pub fn evaluate(&self, environ: &mut StackEnvironment) -> Result<Object, String>{
        match self{
            Statement::Expression(expr) => expr.expression.evaluate(environ),
            Statement::Block(block_stmt) => block_stmt.evaluate(environ),
            Statement::Let(let_stmt) => {
                let value = let_stmt.value.evaluate(environ)?;
                environ.set(&let_stmt.name.value, value);
                Ok(Object::Null(Null {  }))
            },
            Statement::Return(return_stmt) => {
                let val = return_stmt.value.evaluate(environ)?;
                
                Ok(Object::ReturnVal(ReturnValue{
                    value: Box::new(val)
                }))
            }
        }
    }
}

impl Program{

    pub fn evaluate(&self) -> Result<Object, String>{
        let mut result = Object::Null(Null{});
        let mut environ = StackEnvironment::new();

        for stmt in self.statements.iter(){
            result = stmt.evaluate(&mut environ)?;

            if let Object::ReturnVal(ret_val) = result{
                return Ok(*ret_val.value);
            }
        }

        Ok(result)
    }
}
