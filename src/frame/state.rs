use crate::{
    frame::state::{
        array_state::ArrayState, await_state::AwaitState, call_state::CallState, if_state::IfState,
        index_state::IndexState,
    },
    object::ObjectRef,
};
pub mod array_state;
pub mod await_state;
pub mod call_state;
pub mod if_state;
pub mod index_state;

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
    Unary {
        value: Option<ObjectRef>,
        ready_to_evaluate: bool,
    },
    Array {
        ready_to_evaluate: bool,
        state: ArrayState,
    },

    Index {
        ready_to_evaluate: bool,
        state: IndexState,
    },

    If {
        value: Option<ObjectRef>,
        state: IfState,
    },

    Primitive,
}
