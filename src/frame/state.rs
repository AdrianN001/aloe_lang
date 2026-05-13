use crate::{
    frame::state::{
        array_state::ArrayState, await_state::AwaitState, call_state::CallState,
        for_state::ForState, hashmap_state::HashMapState, if_state::IfState,
        index_state::IndexState, infix_state::InfixState, member_state::MemberState,
        value_assign_state::ValueAssignState, while_state::WhileState,
    },
    object::ObjectRef,
};
pub mod array_state;
pub mod await_state;
pub mod call_state;
pub mod for_state;
pub mod hashmap_state;
pub mod if_state;
pub mod index_state;
pub mod infix_state;
pub mod member_state;
pub mod value_assign_state;
pub mod while_state;

#[derive(Debug, Clone)]
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

    HashMap {
        ready_to_evaluate: bool,
        state: HashMapState,
    },

    Infix {
        ready_to_evaluate: bool,
        state: InfixState,
    },

    While {
        value: Option<ObjectRef>,
        state: WhileState,
    },

    For {
        value: Option<ObjectRef>,
        state: ForState,
    },

    Member {
        value: Option<ObjectRef>,
        state: MemberState,
    },

    ValueAssign {
        state: ValueAssignState,
    },

    Primitive,
}
