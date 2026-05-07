use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{expression::Expression, statement::Statement},
    frame::expr_frame::{EvaluationResult, ExprFrameRef, ExpressionFrame},
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        future::task_kind::TaskKind,
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

pub type TaskRef = Rc<RefCell<Task>>;

#[derive(Clone, Default, Debug)]
pub struct Task {
    pub statement_index: usize,
    pub statements: Vec<Statement>,

    pub name: String,
    pub last_object: Option<ObjectRef>,

    pub kind: Option<TaskKind>,

    pub environ: EnvRef,
    pub state: StateRef,

    pub pending_future: Option<ObjectRef>,
    pub result_future: Option<ObjectRef>,

    pub expr_stack: Vec<ExprFrameRef>,
}
impl Task {
    pub fn run(self_ref: TaskRef) -> Result<ObjectRef, RuntimeSignal> {
        loop {
            let frame_opt = {
                let task = self_ref.borrow();
                task.expr_stack.last().cloned()
            };

            if let Some(frame_ref) = frame_opt {
                let result = {
                    let mut frame = frame_ref.borrow_mut();
                    let task = self_ref.borrow();

                    frame.eval_step(task.environ.clone(), task.state.clone())?
                };

                match result {
                    EvaluationResult::Pending => {
                        return Err(RuntimeSignal::Yield(self_ref.clone()));
                    }

                    EvaluationResult::Push(expression) => {
                        let mut task = self_ref.borrow_mut();
                        task.build_frame_from_expr(&expression);

                        continue;
                    }

                    EvaluationResult::Done(obj) => {
                        {
                            let mut task = self_ref.borrow_mut();
                            task.expr_stack.pop();
                        }

                        let parent_opt = {
                            let task = self_ref.borrow();
                            task.expr_stack.last().cloned()
                        };

                        if let Some(parent_ref) = parent_opt {
                            let mut parent_raw = parent_ref.borrow_mut();

                            let interpreter_state = {
                                let task = self_ref.borrow();
                                task.state.clone()
                            };

                            parent_raw.resume_with(obj, interpreter_state)?;

                            continue;
                        }

                        // ka parent frame mehr -> Statement soweit

                        let mut task = self_ref.borrow_mut();

                        let ret_value_opt = task.handle_statement_after_ready_value(obj)?;

                        if let Some(ret_value) = ret_value_opt {
                            return Ok(ret_value);
                        }

                        continue;
                    }
                }
            }

            // 2. neues Statement starten

            let stmt_opt = {
                let task = self_ref.borrow();

                //TODO: wenn die coroutine fertig ist, aber nix returnt wurde, soll es den letzten Wert zuruckgeben.
                if task.statement_index >= task.statements.len() {
                    return Ok(new_objectref(Object::NULL_OBJECT));
                }

                Some(task.statements[task.statement_index].clone())
            };

            let state = {
                let task = self_ref.borrow();
                task.state.clone()
            };

            match stmt_opt.unwrap() {
                Statement::Let(let_stmt) => {
                    let mut task = self_ref.borrow_mut();
                    task.build_frame_from_expr(&let_stmt.value);
                }
                Statement::Return(return_stmt) => {
                    if let Some(return_value) = return_stmt.value {
                        let mut task = self_ref.borrow_mut();
                        task.build_frame_from_expr(&return_value);
                    } else {
                        return Ok(new_objectref(Object::NULL_OBJECT));
                    }
                }
                Statement::Expression(expr_statement) => {
                    let mut task = self_ref.borrow_mut();
                    task.build_frame_from_expr(&expr_statement.expression);
                }

                Statement::Break(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected break keyword in non-loop context",
                        state,
                    )));
                }

                Statement::Continue(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected continue keyword in non-loop context",
                        state,
                    )));
                }
                _ => todo!(),
            }
        }
    }

    fn handle_statement_after_ready_value(
        &mut self,
        value: ObjectRef,
    ) -> Result<Option<ObjectRef>, RuntimeSignal> {
        let current_statement = &self.statements[self.statement_index];
        match current_statement {
            Statement::Let(let_stmt) => {
                self.environ.borrow_mut().set(&let_stmt.name.value, value);
                self.statement_index += 1;
                Ok(None)
            }
            Statement::Return(_) => {
                self.statement_index += 1;
                Ok(Some(value))
            }
            _ => {
                self.statement_index += 1;
                Ok(None)
            }
        }
    }

    fn build_frame_from_expr(&mut self, expression: &Expression) {
        match expression {
            Expression::AwaitExpr(await_expr) => {
                let new_frame = ExpressionFrame::new_await_frame(expression.clone()).to_ref();

                self.expr_stack.push(new_frame);

                self.build_frame_from_expr(&await_expr.expr);
            }
            Expression::Array(_) => {
                let new_frame = ExpressionFrame::new_array_frame(expression.clone()).to_ref();

                self.expr_stack.push(new_frame);
            }
            Expression::Call(_) => {
                let new_frame =
                    ExpressionFrame::new_functioncall_frame(expression.clone()).to_ref();

                self.expr_stack.push(new_frame);
            }
            Expression::Index(_) => {
                let new_frame = ExpressionFrame::new_index_frame(expression.clone()).to_ref();
                self.expr_stack.push(new_frame);
            }
            Expression::IntegerLiteral(_)
            | Expression::Bool(_)
            | Expression::FloatLiteral(_)
            | Expression::String(_)
            | Expression::Identifier(_)
            | Expression::Function(_)
            | Expression::AsyncFunction(_) => {
                let new_frame = ExpressionFrame::new_primitive(expression.clone()).to_ref();

                self.expr_stack.push(new_frame);
            }
            Expression::Prefix(prefix_expr) => {
                let new_frame = ExpressionFrame::new_unary_frame(expression.clone()).to_ref();

                self.expr_stack.push(new_frame);
                self.build_frame_from_expr(&prefix_expr.right.clone());
            }
            other_type => panic!("{}", other_type.to_string()),
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.statement_index == other.statement_index
            && self.statements == other.statements
    }
}

impl Eq for Task {}
