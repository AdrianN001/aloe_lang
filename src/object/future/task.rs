use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::statement::Statement,
    frame::{
        Frame,
        block_frame::BlockFrame,
        expr_frame::EvaluationResult,
        state::ExpressionState,
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

    pub result_future: Option<ObjectRef>,

    pub frames: Vec<(Frame, EnvRef)>,
}
impl Task {
    pub fn new(
        statements: &[Statement],
        name: String,
        environ: EnvRef,
        state: StateRef,
        result_future: ObjectRef,
    ) -> Self {
        let mut new_task = Self {
            statements: statements.to_vec(),
            statement_index: 0,
            name,
            last_object: None,
            kind: None,
            environ,
            state,
            result_future: Some(result_future),
            frames: vec![],
        };

        new_task.init();

        new_task
    }

    pub fn run2(self_ref: TaskRef) -> Result<ObjectRef, RuntimeSignal> {
        let (_environ, interpreter_state) = {
            let task = self_ref.borrow();

            (task.environ.clone(), task.state.clone())
        };

        loop {
            let (current_frame, corresponding_environment) = {
                let task = self_ref.borrow();

                match task.frames.last() {
                    Some(frame) => frame.clone(),
                    None => return Ok(new_objectref(Object::NULL_OBJECT)),
                }
            };

            let eval_result = {
                current_frame
                    .eval_step(corresponding_environment.clone(), interpreter_state.clone())?
            };

            match eval_result {
                EvaluationResult::Push(child_frame) => {
                    self_ref.borrow_mut().frames.push(child_frame);
                }
                EvaluationResult::Pending => {
                    return Err(RuntimeSignal::Yield(self_ref.clone()));
                }
                EvaluationResult::Return(return_value) => return Ok(return_value),
                EvaluationResult::Done(value) => {
                    {
                        self_ref.borrow_mut().frames.pop();
                    }

                    let parent_frame_opt = {
                        let task = self_ref.borrow();

                        task.frames.last().cloned()
                    };

                    if let Some((parent_frame, _env)) = parent_frame_opt {
                        let return_val_opt =
                            parent_frame.resume_with(value, interpreter_state.clone())?;

                        if let Some(return_val) = return_val_opt {
                            return Ok(return_val.clone());
                        }
                    } else {
                        return Ok(value);
                    }
                }
                EvaluationResult::Break(value) => {
                    {
                        self_ref.borrow_mut().frames.pop();
                    }

                    // suche nach dem erste loop frame und resume mit break value

                    loop {
                        let parent_frame_opt = {
                            let task = self_ref.borrow();

                            task.frames.last().cloned()
                        };

                        if let Some((parent_frame, _env)) = parent_frame_opt {
                            match &parent_frame {
                                Frame::BlockFrame(_) => {
                                    self_ref.borrow_mut().frames.pop();
                                    continue;
                                }
                                Frame::ExpressionFrame(expr_frame) => {
                                    let expr_frame_state = {
                                        let expr_frame_raw = expr_frame.borrow();
                                        expr_frame_raw.state.clone()
                                    };

                                    match expr_frame_state {
                                        ExpressionState::While { .. } => {
                                            parent_frame
                                                .resume_with(value, interpreter_state.clone())?;
                                            break;
                                        }
                                        _ => {
                                            self_ref.borrow_mut().frames.pop();
                                            continue;
                                        }
                                    }
                                }
                            };
                        }
                    }
                }
                EvaluationResult::Continue => {
                    {
                        self_ref.borrow_mut().frames.pop();
                    }

                    // suche nach dem erste loop frame und resume mit break value

                    loop {
                        let parent_frame_opt = {
                            let task = self_ref.borrow();

                            task.frames.last().cloned()
                        };

                        if let Some((parent_frame, _env)) = parent_frame_opt {
                            match &parent_frame {
                                Frame::BlockFrame(_) => {
                                    self_ref.borrow_mut().frames.pop();
                                    continue;
                                }
                                Frame::ExpressionFrame(expr_frame) => {
                                    let expr_frame_state = {
                                        let expr_frame_raw = expr_frame.borrow();
                                        expr_frame_raw.state.clone()
                                    };

                                    match expr_frame_state {
                                        ExpressionState::While { .. } => {
                                            parent_frame.resume_with(
                                                new_objectref(Object::NULL_OBJECT),
                                                interpreter_state.clone(),
                                            )?;
                                            break;
                                        }
                                        _ => {
                                            self_ref.borrow_mut().frames.pop();
                                            continue;
                                        }
                                    }
                                }
                            };
                        }
                    }
                }
            }
        }
    }

    fn init(&mut self) {
        let main_block = BlockFrame::new(&self.statements, self.environ.clone());

        self.frames
            .push((Frame::BlockFrame(main_block.to_ref()), self.environ.clone()));
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
