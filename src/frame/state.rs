use crate::{
    frame::state::{await_state::AwaitState, call_state::CallState},
    object::ObjectRef,
};
pub mod await_state;
pub mod call_state;

#[derive(Debug)]
pub enum ExpressionState {
    Await {
        future: Option<ObjectRef>,
        state: AwaitState,
    },
    Call {
        ready_to_evaluate: bool,
        state: CallState,
    },

    Primitive,
}
