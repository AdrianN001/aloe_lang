mod array_expression;
mod async_function_eval;
mod await_expr;
mod block_statement;
mod call_expr;
mod float_obj;
mod for_loop;
mod function_statement;
mod hash_literal;
mod identifier;
mod if_expression;
mod import_statement;
mod index_expr;
mod infix_expr;
mod member_expr;
mod prefix_expr;
mod struct_statement;
mod value_assign;
mod while_loop;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::program::Program;
use crate::module::module_loader::ModuleLoader;
use crate::object::break_value::BreakValue;
use crate::object::error::panic_type::PanicType;
use crate::object::function::Function;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::panic_obj::{PanicObj, RuntimeSignal};
use crate::object::return_value::ReturnValue;
use crate::object::stack_environment::{EnvRef, StackEnvironment};
use crate::object::state::{DEFAULT_INTERPRETER_STATE, StateRef};
use crate::object::string_obj::StringObj;
use crate::object::{ObjectRef, new_objectref};

use super::object::Object;

use super::ast::expression::Expression;
use super::ast::statement::Statement;

impl Expression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        match self {
            Expression::IntegerLiteral(literal) => Ok(new_objectref(Object::Int(Integer {
                value: literal.value,
            }))),
            Expression::FloatLiteral(float_literal) => Ok(float_literal.evaluate()),
            Expression::Identifier(identifier) => identifier.evaluate(environ, state),
            Expression::Bool(bool_literal) => Ok(new_objectref(Object::get_native_boolean_object(
                bool_literal.value,
            ))),
            Expression::Prefix(prefix_expr) => {
                let right_side = prefix_expr.right.evaluate(environ, state.clone())?;

                right_side
                    .borrow_mut()
                    .evaluate_prefix(&prefix_expr.operator, state)
            }
            Expression::ValueAssign(value_assign) => value_assign.evaluate(environ, state),
            Expression::HashMapLiteral(hashmap) => hashmap.evaluate(environ, state),
            Expression::Index(indx_expr) => indx_expr.evaluate(environ, state),
            Expression::String(str_exr) => Ok(new_objectref(Object::String(StringObj {
                value: str_exr.value.clone(),
            }))),
            Expression::Function(func_expr) => Ok(new_objectref(Object::Func(
                Function::from_function_expression(func_expr, environ.clone()),
            ))),
            Expression::AsyncFunction(async_func_expr) => async_func_expr.evaluate(environ),
            Expression::ForLoop(for_loop) => for_loop.evaluate(environ, state),
            Expression::Call(call_expr) => call_expr.evaluate(environ, state),
            Expression::Array(array) => array.evaluate(environ, state),
            Expression::If(if_expression) => if_expression.evaluate(environ, state),
            Expression::Infix(infix_expr) => infix_expr.evaluate_infix_expression(environ, state),
            Expression::Member(member_expression) => member_expression.evaluate(environ, state),
            Expression::WhileLoop(while_loop) => while_loop.evaluate(environ, state),

            Expression::AwaitExpr(await_expr) => await_expr.evaluate(environ, state),

            Expression::InvalidExpression => {
                panic!("unexpected expression type")
            }
        }
    }
}

impl Statement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        match self {
            Statement::Expression(expr) => expr.expression.evaluate(environ, state),
            Statement::Block(block_stmt) => block_stmt.evaluate(environ, state),
            Statement::Let(let_stmt) => {
                let value = let_stmt.value.evaluate(environ.clone(), state)?;

                environ
                    .borrow_mut()
                    .set_to_lowest_level(&let_stmt.name.value, value.clone());
                Ok(new_objectref(Object::NULL_OBJECT))
            }
            Statement::Return(return_stmt) => {
                let val = match &return_stmt.value {
                    Some(return_value) => return_value.evaluate(environ, state)?,
                    None => new_objectref(Object::NULL_OBJECT),
                };
                if let Object::ReturnVal(ret_val) = &*val.borrow() {
                    return Ok(ret_val.unwrap_to_value().clone());
                }

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
            Statement::Struct(struct_stmt) => struct_stmt.evaluate(environ, state),
            Statement::AsyncFunction(async_func_stmt) => async_func_stmt.evaluate(environ),
            Statement::Import(_) => panic!("already catched"),
        }
    }
}

impl Program {
    pub fn evaluate(
        &self,
        environ: EnvRef,
        module_loader: &mut ModuleLoader,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));
        let state = Rc::new(RefCell::new(DEFAULT_INTERPRETER_STATE));

        for stmt in self.statements.iter() {
            result = match stmt {
                Statement::Import(import_stmt) => {
                    import_stmt.evaluate(environ.clone(), state.clone(), module_loader)?
                }
                other_stmt => other_stmt.evaluate(environ.clone(), state.clone())?,
            };

            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::BreakVal(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected break keyword in non-loop context",
                        state,
                    )));
                }
                Object::Continue => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected continue keyword in non-loop context",
                        state,
                    )));
                }
                Object::ReturnVal(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::ReturnFromNonfunctionalContext,
                        "unexpected return keyword in non-function context",
                        state,
                    )));
                }
                _ => {}
            }
        }

        {
            let mut state_borrow_mut = state.borrow_mut();
            state_borrow_mut.scheduler.run();
        }

        Ok(result)
    }

    pub fn evaluate_as_repl(&self, environ: EnvRef) -> Result<ObjectRef, RuntimeSignal> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));
        let state = Rc::new(RefCell::new(DEFAULT_INTERPRETER_STATE));

        for stmt in self.statements.iter() {
            result = match stmt {
                Statement::Import(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::ImportUnsupported,
                        "import is not supported in repl",
                        state,
                    )));
                }
                other_stmt => other_stmt.evaluate(environ.clone(), state.clone())?,
            };

            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::BreakVal(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected break keyword in non-loop context",
                        state,
                    )));
                }
                Object::Continue => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected continue keyword in non-loop context",
                        state,
                    )));
                }
                Object::ReturnVal(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::ReturnFromNonfunctionalContext,
                        "unexpected return keyword in non-function context",
                        state,
                    )));
                }
                _ => {}
            }
        }

        Ok(result)
    }

    pub fn evaluate_with_default(&self) -> Result<ObjectRef, RuntimeSignal> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));
        let state = Rc::new(RefCell::new(DEFAULT_INTERPRETER_STATE));
        let environ = Rc::new(RefCell::new(StackEnvironment::new()));

        for stmt in self.statements.iter() {
            result = match stmt {
                Statement::Import(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::ImportUnsupported,
                        "import is not supported in repl",
                        state,
                    )));
                }
                other_stmt => other_stmt.evaluate(environ.clone(), state.clone())?,
            };

            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::BreakVal(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected break keyword in non-loop context",
                        state,
                    )));
                }
                Object::Continue => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected continue keyword in non-loop context",
                        state,
                    )));
                }
                Object::ReturnVal(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::ReturnFromNonfunctionalContext,
                        "unexpected return keyword in non-function context",
                        state,
                    )));
                }
                _ => {}
            }
        }

        {
            let mut state_borrow_mut = state.borrow_mut();
            state_borrow_mut.scheduler.run();
        }

        Ok(result)
    }
}
