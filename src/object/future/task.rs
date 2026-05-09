use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::statement::Statement,
    frame::{Frame, block_frame::BlockFrame, expr_frame::EvaluationResult},
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

    pub frames: Vec<Frame>,
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
        let (environ, interpreter_state) = {
            let task = self_ref.borrow();

            (task.environ.clone(), task.state.clone())
        };

        loop {
            let current_frame = {
                let task = self_ref.borrow();

                match task.frames.last() {
                    Some(frame) => frame.clone(),
                    None => return Ok(new_objectref(Object::NULL_OBJECT)),
                }
            };

            let eval_result =
                { current_frame.eval_step(environ.clone(), interpreter_state.clone())? };

            match eval_result {
                //TODO:CRAZY
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

                    if let Some(parent_frame) = parent_frame_opt {
                        let return_val_opt =
                            parent_frame.resume_with(value, interpreter_state.clone())?;

                        if let Some(return_val) = return_val_opt {
                            return Ok(return_val.clone());
                        }
                    } else {
                        return Ok(value);
                    }
                }
            }
        }
    }

    fn init(&mut self) {
        let main_block = BlockFrame {
            statements: self.statements.clone(),
            index: 0,
            last_value: None,
            environ: self.environ.clone(),
            last_object: None,
        };

        self.frames.push(Frame::BlockFrame(main_block.to_ref()))
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
