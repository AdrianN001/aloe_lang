use crate::{
    frame::{
        block_frame::BlockFrameRef,
        expr_frame::{EvaluationResult, ExprFrameRef},
    },
    object::{ObjectRef, panic_obj::RuntimeSignal, stack_environment::EnvRef, state::StateRef},
};

pub mod block_frame;
pub mod evaluation;
pub mod expr_frame;
pub mod state;

#[derive(Clone, Debug)]
pub enum Frame {
    ExpressionFrame(ExprFrameRef),
    BlockFrame(BlockFrameRef),
}

impl Frame {
    pub fn eval_step(
        &self,
        environ: EnvRef,
        interpreter_state: StateRef,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        match self {
            Frame::ExpressionFrame(expr_frame) => expr_frame
                .borrow_mut()
                .eval_step(environ, interpreter_state),
            Frame::BlockFrame(block_frame) => Ok(block_frame.borrow_mut().eval_step()),
        }
    }

    pub fn resume_with(
        &self,
        value: ObjectRef,
        interpreter_state: StateRef,
    ) -> Result<Option<ObjectRef>, RuntimeSignal> {
        match self {
            Frame::ExpressionFrame(expr_frame) => {
                expr_frame
                    .borrow_mut()
                    .resume_with(value, interpreter_state)?;
                Ok(None)
            }
            Frame::BlockFrame(block_frame) => Ok(block_frame.borrow_mut().resume_with(value)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Frame::BlockFrame(b) => format!("block frame {:?}", b.borrow().statements),
            Frame::ExpressionFrame(f) => format!("expression frame {:?}", f.borrow().expr),
        }
    }
}
