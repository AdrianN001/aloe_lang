use crate::{
    ast::expression::Expression,
    frame::expr_frame::EvaluationResult,
    object::{
        Object, ObjectRef, future::future_state::FutureState, panic_obj::RuntimeSignal,
        stack_environment::EnvRef, state::StateRef,
    },
};

#[derive(Debug)]
pub enum AwaitState {
    Start,
    Waiting,
    Done,
}

impl AwaitState {
    pub fn eval_step(
        &mut self,
        expression: &Expression,
        future_saved_in_frame: &mut Option<ObjectRef>,
        _environ: EnvRef,
        _state: StateRef,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        let _await_expr = {
            match expression {
                Expression::AwaitExpr(await_expr) => await_expr,
                other => panic!("{}", other.to_string()),
            }
        };

        match self {
            AwaitState::Start => unreachable!(),
            AwaitState::Waiting => {
                let future = future_saved_in_frame.as_ref().unwrap();

                let future_borrow = future.borrow();

                let future_raw = {
                    match &*future_borrow {
                        Object::Future(future_raw) => future_raw,
                        other_type => panic!("{}", other_type.get_type()),
                    }
                };

                match &future_raw.state {
                    FutureState::Ready(value) => Ok(EvaluationResult::Done(value.clone())),
                    FutureState::Pending(_) => Ok(EvaluationResult::Pending),
                    _ => panic!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
