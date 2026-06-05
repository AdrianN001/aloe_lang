mod array_expression;
mod async_function_eval;
mod await_expr;
mod block_statement;
mod break_stmt;
mod call_expr;
mod continue_stmt;
mod enum_statement;
mod expression_stmt;
mod float_obj;
mod for_loop;
mod function_statement;
mod hash_literal;
mod identifier;
mod if_expression;
mod import_statement;
mod index_expr;
mod infix_expr;
mod launch_stmt;
mod let_statement;
mod member_expr;
mod prefix_expr;
mod return_stmt;
mod scope_resolution_expr;
mod string_literal;
mod struct_statement;
mod value_assign;
mod while_loop;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::program::Program;
use crate::module::module_loader::ModuleLoader;
use crate::object::error::panic_type::PanicType;
use crate::object::function::Function;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::panic_obj::{PanicObj, RuntimeSignal};
use crate::object::stack_environment::{EnvRef, StackEnvironment};
use crate::object::state::{InterpreterState, StateRef};
use crate::object::{ObjectRef, new_objectref};
use crate::scheduler::GLOBAL_SCHEDULER;

use super::object::Object;

use super::ast::expression::Expression;
use super::ast::statement::Statement;

impl Expression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        match self {
            Expression::IntegerLiteral(literal) => Ok(new_objectref(Object::Int(Integer {
                value: literal.value,
            }))),
            Expression::Null(_) => Ok(new_objectref(Object::NULL_OBJECT)),
            Expression::FloatLiteral(float_literal) => Ok(float_literal.evaluate()),
            Expression::Identifier(identifier) => identifier.evaluate(environ, state),
            Expression::Bool(bool_literal) => Ok(new_objectref(Object::get_native_boolean_object(
                bool_literal.value,
            ))),
            Expression::Prefix(prefix_expr) => {
                let right_side = prefix_expr.right.evaluate(environ, state.clone())?;

                right_side.borrow_mut().evaluate_prefix(
                    &prefix_expr.operator,
                    prefix_expr.token.line_number,
                    state,
                )
            }
            Expression::ValueAssign(value_assign) => value_assign.evaluate(environ, state),
            Expression::HashMapLiteral(hashmap) => hashmap.evaluate(environ, state),
            Expression::Index(indx_expr) => indx_expr.evaluate(environ, state),
            Expression::String(str_exr) => str_exr.evaluate(environ, state),
            Expression::Function(func_expr) => Ok(new_objectref(Object::Func(Box::new(
                Function::from_function_expression(func_expr, environ.clone()),
            )))),
            Expression::AsyncFunction(async_func_expr) => async_func_expr.evaluate(environ),
            Expression::ForLoop(for_loop) => for_loop.evaluate(environ, state),
            Expression::Call(call_expr) => call_expr.evaluate(environ, state),
            Expression::Array(array) => array.evaluate(environ, state),
            Expression::If(if_expression) => if_expression.evaluate(environ, state),
            Expression::Infix(infix_expr) => infix_expr.evaluate_infix_expression(environ, state),
            Expression::Member(member_expression) => member_expression.evaluate(environ, state),
            Expression::WhileLoop(while_loop) => while_loop.evaluate(environ, state),
            Expression::ScopeResolution(scope_resolution) => {
                scope_resolution.evaluate(environ, state)
            }

            Expression::AwaitExpr(_) => Err(RuntimeSignal::Panic(PanicObj::new_simple(
                PanicType::AwaitedInNonAsyncContext,
                "await expression is not allowed in non-async context",
                state,
            ))),

            Expression::InvalidExpression => {
                panic!("unexpected expression type")
            }
        }
    }
}

impl Statement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        match self {
            Statement::Expression(expr_stmt) => expr_stmt.evaluate(environ, state),
            Statement::Block(block_stmt) => block_stmt.evaluate(environ, state),
            Statement::Let(let_stmt) => let_stmt.evaluate(environ, state),
            Statement::Return(return_stmt) => return_stmt.evaluate(environ, state),
            Statement::Continue(continue_stmt) => continue_stmt.evaluate(environ, state),
            Statement::Break(break_stmt) => break_stmt.evaluate(environ, state),
            Statement::Function(func_stmt) => Ok(func_stmt.evaluate(environ.clone())),
            Statement::Struct(struct_stmt) => struct_stmt.evaluate(environ, state),
            Statement::AsyncFunction(async_func_stmt) => async_func_stmt.evaluate(environ),
            Statement::Launch(launch_stmt) => launch_stmt.evaluate(environ, state),
            Statement::Enum(enum_stmt) => enum_stmt.evaluate(environ, state),
            Statement::Import(_) => Err(RuntimeSignal::Panic(PanicObj::new_simple(
                PanicType::WrongSyntax,
                "import is only allowed in the top of the file.",
                state.clone(),
            ))),
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
        let state = Rc::new(RefCell::new(InterpreterState::default()));

        for stmt in self.statements.iter() {
            result = match stmt {
                Statement::Import(import_stmt) => {
                    import_stmt.evaluate(environ.clone(), state.clone(), module_loader)?
                }
                other_stmt => match other_stmt.evaluate(environ.clone(), state.clone()) {
                    Ok(result) => result,
                    Err(RuntimeSignal::Return(_val)) => {
                        return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                            PanicType::UnexpectedKeyword,
                            "unexpected return statement outside of function",
                            state.clone(),
                        )));
                    }

                    other => return other,
                },
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

                _ => {}
            }
        }

        GLOBAL_SCHEDULER.with(|scheduler| {
            let mut scheduler_borrow = scheduler.borrow_mut();
            scheduler_borrow.run()?;
            Ok(())
        })?;

        Ok(result)
    }

    pub fn evaluate_as_repl(&self, environ: EnvRef) -> Result<ObjectRef, RuntimeSignal> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));
        let state = Rc::new(RefCell::new(InterpreterState::default()));

        for stmt in self.statements.iter() {
            result = match stmt {
                Statement::Import(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::ImportUnsupported,
                        "import is not supported in repl",
                        state,
                    )));
                }
                other_stmt => match other_stmt.evaluate(environ.clone(), state.clone()) {
                    Ok(result) => result,
                    Err(RuntimeSignal::Return(_val)) => {
                        return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                            PanicType::UnexpectedKeyword,
                            "unexpected return statement outside of function",
                            state.clone(),
                        )));
                    }

                    other => return other,
                },
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

                _ => {}
            }
        }

        Ok(result)
    }

    pub fn evaluate_with_default(&self) -> Result<ObjectRef, RuntimeSignal> {
        let mut result = Rc::new(RefCell::new(Object::Null(Null {})));
        let state = Rc::new(RefCell::new(InterpreterState::default()));
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
                other_stmt => match other_stmt.evaluate(environ.clone(), state.clone()) {
                    Ok(result) => result,
                    Err(RuntimeSignal::Return(_val)) => {
                        return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                            PanicType::UnexpectedKeyword,
                            "unexpected return statement outside of function",
                            state.clone(),
                        )));
                    }

                    other => return other,
                },
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

                _ => {}
            }
        }

        //TODO: check if scheduler panics and return the panic instead of panicking here
        GLOBAL_SCHEDULER.with(|scheduler| {
            let mut scheduler_borrow = scheduler.borrow_mut();
            scheduler_borrow.run()?;
            Ok(())
        })?;

        Ok(result)
    }
}
