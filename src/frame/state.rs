use crate::{frame::state::await_state::AwaitState, object::ObjectRef};
pub mod await_state;

#[derive(Debug)]
pub enum ExpressionState {
    Await {
        future: Option<ObjectRef>,
        state: AwaitState,
    },

    Primitive,
}
