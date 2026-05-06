use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{expression::Expression, statement::Statement},
    frame::{
        self,
        expr_frame::{EvaluationResult, ExprFrameRef, ExpressionFrame},
    },
    object::{
        Object, ObjectRef, future::task_kind::TaskKind, new_objectref, panic_obj::RuntimeSignal,
        stack_environment::EnvRef, state::StateRef,
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
                            //TODO!: wenn es eine Elternframe gab, dann soll der Wert ihr zuruckgeben (parentref.borrow().resume_with)

                            continue;
                        }

                        // ka parent frame mehr -> Statement soweit

                        let mut task = self_ref.borrow_mut();

                        let ret_value_opt = task.handle_statement_after_ready_value(obj);

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
                _ => todo!(),
            }
        }
    }

    fn handle_statement_after_ready_value(&mut self, value: ObjectRef) -> Option<ObjectRef> {
        let current_statement = &self.statements[self.statement_index];
        match current_statement {
            Statement::Let(let_stmt) => {
                self.environ.borrow_mut().set(&let_stmt.name.value, value);
                self.statement_index += 1;
                None
            }
            Statement::Return(_) => {
                self.statement_index += 1;
                Some(value)
            }
            _ => {
                self.statement_index += 1;
                None
            }
        }
    }

    fn build_frame_from_expr(&mut self, expression: &Expression) {
        match expression {
            Expression::AwaitExpr(await_expr) => {
                let new_frame =
                    ExpressionFrame::new_await_frame(Expression::AwaitExpr(await_expr.clone()))
                        .to_ref();

                self.expr_stack.push(new_frame);
            }
            Expression::IntegerLiteral(integer_ltr) => {
                let new_frame =
                    ExpressionFrame::new_primitive(Expression::IntegerLiteral(integer_ltr.clone()))
                        .to_ref();

                self.expr_stack.push(new_frame);
            }
            _ => panic!(),
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
