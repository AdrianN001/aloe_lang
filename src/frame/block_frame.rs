use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::statement::Statement,
    frame::expr_frame::{EvaluationResult, ExpressionFrame},
    object::{
        Object, ObjectRef,
        break_value::BreakValue,
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
        }
    }

    pub fn set_loop_context(&mut self, is_loop_context: bool) {
        self.environ.borrow_mut().set_loop_context(is_loop_context);
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

    pub fn resume_with(&mut self, value: ObjectRef) -> Option<ObjectRef> {
        self.handle_statement_after_ready_value(value.clone())
    }

    fn eval_current_stmt(
        &mut self,
        statement: &Statement,
        state: StateRef,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        match statement {
            Statement::Let(let_stmt) => Ok(ExpressionFrame::build_frame_from_expr(
                &let_stmt.value,
                self.environ.clone(),
            )),
            Statement::Return(return_stmt) => {
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
            Statement::Continue(_) => {
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

            _ => todo!(),
        }
    }

    fn handle_statement_after_ready_value(&mut self, value: ObjectRef) -> Option<ObjectRef> {
        let current_statement = &self.statements[self.index];
        match current_statement {
            Statement::Let(let_stmt) => {
                self.environ
                    .borrow_mut()
                    .set(&let_stmt.name.value, value.clone());
                self.index += 1;
                None
            }
            Statement::Return(_) => {
                self.index += 1;
                Some(value)
            }
            Statement::Break(_) => {
                self.break_value = Some(new_objectref(Object::BreakVal(BreakValue {
                    value: Box::new(value.clone()),
                })));
                None
            }
            Statement::Continue(_) => None,

            _ => {
                self.last_object = Some(value.clone());
                self.index += 1;
                None
            }
        }
    }

    pub fn to_ref(self) -> BlockFrameRef {
        Rc::new(RefCell::new(self))
    }
}
