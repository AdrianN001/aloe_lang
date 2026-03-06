mod block_statement;
mod call_expr;
mod float_obj;
mod for_loop;
mod function_statement;
mod hash_literal;
mod identifier;
mod if_expression;
mod index_expr;
mod infix_expr;
mod member_expr;
mod prefix_expr;
mod value_assign;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::program::Program;
use crate::object::ObjectRef;
use crate::object::array::Array;
use crate::object::break_value::BreakValue;
use crate::object::function::Function;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::return_value::ReturnValue;
use crate::object::stack_environment::{EnvRef, StackEnvironment};
use crate::object::state::{DEFAULT_INTERPRETER_STATE, InterpreterState, StateRef};
use crate::object::string_obj::StringObj;

use super::object::Object;

use super::ast::expression::Expression;
use super::ast::statement::Statement;

impl Expression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, String> {
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
                let right_side = prefix_expr.right.evaluate(environ.clone(), state)?;

                right_side
                    .borrow_mut()
                    .evaluate_prefix(&prefix_expr.operator)
            }
            Expression::ValueAssign(value_assign) => value_assign.evaluate(environ.clone(), state),
            Expression::HashMapLiteral(hashmap) => hashmap.evaluate(environ.clone(), state),
            Expression::Index(indx_expr) => indx_expr.evaluate(environ.clone(), state),
            Expression::String(str_exr) => Ok(Rc::new(RefCell::new(Object::String(StringObj {
                value: str_exr.value.clone(),
            })))),
            Expression::Function(func_expr) => Ok(Rc::new(RefCell::new(Object::Func(
                Function::from_function_expression(func_expr, environ.clone()),
            )))),
            Expression::ForLoop(for_loop) => for_loop.evaluate(environ.clone(), state),
            Expression::Call(call_expr) => call_expr.evaluate(environ, state),
            Expression::Array(array) => {
                let mut objects = Vec::new();

                for element in &array.elements {
                    objects.push(element.evaluate(environ.clone(), state.clone())?.clone());
                }

                Ok(Rc::new(RefCell::new(Object::Array(Array {
                    items: objects,
                }))))
            }
            Expression::If(if_expression) => if_expression.evaluate(environ.clone(), state),
            Expression::Infix(infix_expr) => {
                let right_side = infix_expr.right.evaluate(environ.clone(), state.clone())?;
                let left_side = infix_expr.left.evaluate(environ.clone(), state.clone())?;

                left_side
                    .borrow()
                    .evaluate_infix_expression(right_side.clone(), &infix_expr.operator)
            }
            Expression::Member(member_expression) => {
                member_expression.evaluate(environ.clone(), state)
            }
            Expression::InvalidExpression => {
                panic!("unexpected expression type")
            }
        }
    }
}

impl Statement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, String> {
        match self {
            Statement::Expression(expr) => expr.expression.evaluate(environ, state),
            Statement::Block(block_stmt) => block_stmt.evaluate(environ, state),
            Statement::Let(let_stmt) => {
                let value = let_stmt.value.evaluate(environ.clone(), state)?;
                match &*value.borrow() {
                    Object::ReturnVal(ret_val) => {
                        environ
                            .borrow_mut()
                            .set_to_lowest_level(&let_stmt.name.value, *ret_val.value.clone());
                        Ok(*ret_val.value.clone())
                    }
                    _ => {
                        environ
                            .borrow_mut()
                            .set_to_lowest_level(&let_stmt.name.value, value.clone());
                        Ok(value.clone())
                    }
                }
            }
            Statement::Return(return_stmt) => {
                let val = return_stmt.value.evaluate(environ, state)?;

                Ok(Rc::new(RefCell::new(Object::ReturnVal(ReturnValue {
                    value: Box::new(val.clone()),
                }))))
            }
            Statement::Continue(_continue_stmt) => Ok(Rc::new(RefCell::new(Object::Continue))),
            Statement::Break(break_stmt) => {
                let val = if let Some(break_expression_value) = &break_stmt.expression {
                    break_expression_value
                        .evaluate(environ.clone(), state)?
                        .clone()
                } else {
                    Rc::new(RefCell::new(Object::NULL_OBJECT))
                };

                Ok(Rc::new(RefCell::new(Object::BreakVal(BreakValue {
                    value: Box::new(val),
                }))))
            }
            Statement::Function(func_stmt) => Ok(func_stmt.evaluate(environ.clone())),
        }
    }
}

impl Program {
    pub fn evaluate(&self) -> Result<ObjectRef, String> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));
        let environ = Rc::new(RefCell::new(StackEnvironment::new()));
        let state = Rc::new(RefCell::new(DEFAULT_INTERPRETER_STATE));

        for stmt in self.statements.iter() {
            result = stmt.evaluate(environ.clone(), state.clone())?;
            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::BreakVal(_) => {
                    return Err("unexpected break keyword in non-loop context".into());
                }
                Object::Continue => {
                    return Err("unexpected continue keyword in non-loop context".into());
                }
                Object::ReturnVal(ret_val) => return Ok(*ret_val.value.clone()),
                Object::Err(_) => return Ok(result.clone()),
                _ => {}
            }
        }

        Ok(result)
    }

    pub fn evaluate_with_other_environment(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));
        let state = Rc::new(RefCell::new(DEFAULT_INTERPRETER_STATE));

        for stmt in self.statements.iter() {
            result = stmt.evaluate(environ.clone(), state.clone())?;
            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::BreakVal(_) => {
                    return Err("unexpected break keyword in non-loop context".into());
                }
                Object::Continue => {
                    return Err("unexpected continue keyword in non-loop context".into());
                }
                Object::ReturnVal(ret_val) => return Ok(*ret_val.value.clone()),
                Object::Err(_) => return Ok(result.clone()),
                _ => {}
            }
        }

        Ok(result)
    }
}
