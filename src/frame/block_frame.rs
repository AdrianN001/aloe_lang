use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, value_assign_expression::ValueAssignExpression},
        statement::{Statement, let_statement::LetStatement},
    },
    frame::expr_frame::{EvaluationResult, ExpressionFrame},
    object::{
        Object, ObjectRef,
        break_value::BreakValue,
        built_in::async_await::spawn_builtin_function,
        error::panic_type::PanicType,
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::{EnvRef, StackEnvironment},
        state::StateRef,
    },
};

pub type BlockFrameRef = Rc<RefCell<BlockFrame>>;

#[derive(Debug)]
pub struct BlockFrame {
    pub statements: Vec<Statement>,
    pub index: usize,

    pub last_value: Option<ObjectRef>,

    pub environ: EnvRef,
    pub last_object: Option<ObjectRef>,

    break_value: Option<ObjectRef>, //TODO: temporary solution, we should refactor this later
    launch_value: Option<ObjectRef>, //TODO: temporary solution
}

impl BlockFrame {
    pub fn new(statements: &[Statement], base_environment: EnvRef) -> Self {
        let environ = {
            let raw = StackEnvironment::new_enclosed(base_environment, String::from(""));
            Rc::new(RefCell::new(raw))
        };

        Self {
            statements: statements.to_vec(),
            index: 0,

            last_value: None,

            environ,
            last_object: None,
            break_value: None,
            launch_value: None,
        }
    }

    pub fn set_loop_context(&mut self, is_loop_context: bool) {
        self.environ.borrow_mut().set_loop_context(is_loop_context);
    }

    pub fn add_new_variable(&mut self, variable_name: &str, variable_value: ObjectRef) {
        self.environ
            .borrow_mut()
            .set_to_lowest_level(variable_name, variable_value);
    }

    pub fn eval_step(&mut self, state: StateRef) -> Result<EvaluationResult, RuntimeSignal> {
        if self.index >= self.statements.len() {
            return Ok(EvaluationResult::Done(match &self.last_object {
                Some(value) => value.clone(),
                None => new_objectref(Object::NULL_OBJECT),
            }));
        }

        let stmt = self.statements[self.index].clone();
        self.eval_current_stmt(&stmt, state)
    }

    pub fn resume_with(
        &mut self,
        value: ObjectRef,
        state: StateRef,
    ) -> Result<Option<ObjectRef>, RuntimeSignal> {
        self.handle_statement_after_ready_value(value.clone(), state)
    }

    fn eval_current_stmt(
        &mut self,
        statement: &Statement,
        state: StateRef,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        match statement {
            Statement::Let(let_stmt) => {
                {
                    state
                        .borrow_mut()
                        .set_current_line(let_stmt.token.line_number);
                }
                let right_side = match &let_stmt.assignment {
                    Expression::ValueAssign(assignment) => assignment.right.clone(),
                    _ => unreachable!(),
                };
                Ok(ExpressionFrame::build_frame_from_expr(
                    &right_side,
                    self.environ.clone(),
                ))
            }
            Statement::Return(return_stmt) => {
                {
                    state
                        .borrow_mut()
                        .set_current_line(return_stmt.token.line_number);
                }
                //NOTE: Wir mussen die Interpreterstate nicht checken, ob es eine funktion ist, weil await darf eh nur in async funktionen benutzt werden
                if let Some(return_value) = &return_stmt.value {
                    Ok(ExpressionFrame::build_frame_from_expr(
                        &return_value,
                        self.environ.clone(),
                    ))
                } else {
                    return Ok(EvaluationResult::Return(new_objectref(Object::NULL_OBJECT)));
                }
            }
            Statement::Expression(expr_statement) => Ok(ExpressionFrame::build_frame_from_expr(
                &expr_statement.expression,
                self.environ.clone(),
            )),

            Statement::Break(break_stmt) => {
                {
                    state
                        .borrow_mut()
                        .set_current_line(break_stmt.token.line_number);
                }
                let is_loop_context = { self.environ.borrow().is_loop_context() };

                if !is_loop_context {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::UnexpectedKeyword,
                        format!("'break' used outside of loop context"),
                        state,
                    )));
                }

                if let Some(value) = &break_stmt.expression {
                    if let Some(break_value) = &self.break_value {
                        return Ok(EvaluationResult::Break(break_value.clone()));
                    }
                    Ok(ExpressionFrame::build_frame_from_expr(
                        value,
                        self.environ.clone(),
                    ))
                } else {
                    Ok(EvaluationResult::Break(new_objectref(Object::BreakVal(
                        BreakValue {
                            value: Box::new(new_objectref(Object::NULL_OBJECT)),
                        },
                    ))))
                }
            }
            Statement::Continue(continue_stmt) => {
                {
                    state
                        .borrow_mut()
                        .set_current_line(continue_stmt.token.line_number);
                }
                let is_loop_context = { self.environ.borrow().is_loop_context() };

                if !is_loop_context {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::UnexpectedKeyword,
                        format!("'continue' used outside of loop context"),
                        state,
                    )));
                }

                Ok(EvaluationResult::Continue)
            }

            Statement::Launch(launch_stmt) => {
                {
                    state
                        .borrow_mut()
                        .set_current_line(launch_stmt.token.line_number);
                }
                if let Some(launch_value) = &self.launch_value {
                    let result = spawn_builtin_function(&[launch_value.clone()], state)?;
                    self.launch_value = None;
                    self.index += 1;
                    return Ok(EvaluationResult::Done(result));
                } else {
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &launch_stmt.expr,
                        self.environ.clone(),
                    ));
                }
            }

            other_statement => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::IllegalExpression,
                    format!(
                        "'{}' inside of an async function is illegal",
                        other_statement.to_string()
                    ),
                    state,
                )));
            }
        }
    }

    fn handle_statement_after_ready_value(
        &mut self,
        value: ObjectRef,
        state: StateRef,
    ) -> Result<Option<ObjectRef>, RuntimeSignal> {
        let current_statement = &self.statements[self.index];
        match current_statement {
            Statement::Let(let_stmt) => {
                self.handle_let_statement_with_value(let_stmt, value, state)?;
                /*
                self.environ
                    .borrow_mut()
                    .set(&let_stmt.name.value, value.clone());
                */
                self.index += 1;
                Ok(None)
            }
            Statement::Return(_) => {
                self.index += 1;
                Ok(Some(value))
            }
            Statement::Break(_) => {
                self.break_value = Some(new_objectref(Object::BreakVal(BreakValue {
                    value: Box::new(value.clone()),
                })));
                Ok(None)
            }
            Statement::Continue(_) => Ok(None),

            Statement::Launch(_) => {
                self.launch_value = Some(value);
                Ok(None)
            }

            _ => {
                self.last_object = Some(value.clone());
                self.index += 1;
                Ok(None)
            }
        }
    }

    fn handle_let_statement_with_value(
        &self,
        statement: &LetStatement,
        right_value: ObjectRef,
        interpreter_state: StateRef,
    ) -> Result<(), RuntimeSignal> {
        let assignment_expr = match &statement.assignment {
            Expression::ValueAssign(value_assign) => value_assign,
            _ => unreachable!(),
        };

        match &*assignment_expr.left {
            Expression::Identifier(identifier_expr) => {
                self.environ
                    .borrow_mut()
                    .set_to_lowest_level(&identifier_expr.value, right_value);
                Ok(())
            }
            Expression::Array(destruct_arr) => {
                let identifiers = ValueAssignExpression::get_identifier_from_destruct_arr(
                    &destruct_arr,
                    interpreter_state.clone(),
                )?;
                let _ = ValueAssignExpression::evaluate_destructuring_with_let_binding(
                    &identifiers,
                    right_value,
                    self.environ.clone(),
                    interpreter_state,
                )?;
                Ok(())
            }
            other_expression => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::WrongSyntax,
                    format!(
                        "expcted after let keyword an identifier, or an array for destructuring, got: '{}'",
                        other_expression.to_string()
                    ),
                    interpreter_state,
                )));
            }
        }
    }

    pub fn to_ref(self) -> BlockFrameRef {
        Rc::new(RefCell::new(self))
    }
}
