mod block_statement;
mod call_expr;
mod float_obj;
mod hash_literal;
mod identifier;
mod if_expression;
mod index_expr;
mod infix_expr;
mod member_expr;
mod prefix_expr;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::program::Program;
use crate::object::ObjectRef;
use crate::object::array::Array;
use crate::object::function::Function;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::return_value::ReturnValue;
use crate::object::stack_environment::{EnvRef, StackEnvironment};
use crate::object::string_obj::StringObj;

use super::object::Object;

use super::ast::expression::Expression;
use super::ast::statement::Statement;

impl Expression {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        match self {
            Expression::IntegerLiteral(literal) => {
                Ok(Rc::new(RefCell::new(Object::Int(Integer {
                    value: literal.value,
                }))))
            }
            Expression::FloatLiteral(float_literal) => Ok(float_literal.evaluate()),
            Expression::Identifier(identifier) => identifier.evaluate(environ.clone()),
            Expression::Bool(bool_literal) => Ok(Rc::new(RefCell::new(
                Object::get_native_boolean_object(bool_literal.value),
            ))),
            Expression::Prefix(prefix_expr) => {
                let right_side = prefix_expr.right.evaluate(environ.clone())?;

                right_side
                    .borrow_mut()
                    .evaluate_prefix(&prefix_expr.operator)
            }
            Expression::HashMapLiteral(hashmap) => hashmap.evaluate(environ.clone()),
            Expression::Index(indx_expr) => indx_expr.evaluate(environ.clone()),
            Expression::String(str_exr) => Ok(Rc::new(RefCell::new(Object::String(StringObj {
                value: str_exr.value.clone(),
            })))),
            Expression::Function(func_expr) => Ok(Rc::new(RefCell::new(Object::Func(
                Function::from_function_expression(func_expr, environ.clone()),
            )))),
            Expression::Call(call_expr) => {
                let function_object = call_expr.function.evaluate(environ.clone())?.clone();

                let args = call_expr.evaluate_arguments(environ.clone())?;

                match &*function_object.borrow() {
                    Object::Func(function) => function.apply(&args),
                    Object::BuiltIn(built_in_function) => Ok(built_in_function.call(&args)),
                    _ => Err("not a function".into()),
                }
            }
            Expression::Array(array) => {
                let mut objects = Vec::new();

                for element in &array.elements {
                    objects.push(element.evaluate(environ.clone())?.clone());
                }

                Ok(Rc::new(RefCell::new(Object::Array(Array {
                    items: objects,
                }))))
            }
            Expression::If(if_expression) => if_expression.evaluate(environ.clone()),
            Expression::Infix(infix_expr) => {
                let right_side = infix_expr.right.evaluate(environ.clone())?;
                let left_side = infix_expr.left.evaluate(environ.clone())?;

                left_side
                    .borrow()
                    .evaluate_infix_expression(right_side.clone(), &infix_expr.operator)
            }
            Expression::Member(member_expression) => member_expression.evaluate(environ.clone()),
            Expression::InvalidExpression | Expression::ForLoop(_) => {
                panic!("unexpected expression type")
            }
        }
    }
}

impl Statement {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        match self {
            Statement::Expression(expr) => expr.expression.evaluate(environ),
            Statement::Block(block_stmt) => block_stmt.evaluate(environ),
            Statement::Let(let_stmt) => {
                let value = let_stmt.value.evaluate(environ.clone())?;
                environ.borrow_mut().set(&let_stmt.name.value, &value);
                Ok(Rc::new(RefCell::new(Object::Null(Null {}))))
            }
            Statement::Return(return_stmt) => {
                let val = return_stmt.value.evaluate(environ)?;

                Ok(Rc::new(RefCell::new(Object::ReturnVal(ReturnValue {
                    value: Box::new(val.clone()),
                }))))
            }
        }
    }
}

impl Program {
    pub fn evaluate(&self) -> Result<ObjectRef, String> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));
        let environ = Rc::new(RefCell::new(StackEnvironment::new()));

        for stmt in self.statements.iter() {
            result = stmt.evaluate(environ.clone())?;

            if let Object::ReturnVal(ret_val) = &*result.borrow() {
                return Ok(*ret_val.value.clone());
            }
        }

        Ok(result)
    }

    pub fn evaluate_with_other_environment(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));

        for stmt in self.statements.iter() {
            result = stmt.evaluate(environ.clone())?;

            if let Object::ReturnVal(ret_val) = &*result.borrow() {
                return Ok(*ret_val.value.clone());
            }
        }

        Ok(result)
    }
}
