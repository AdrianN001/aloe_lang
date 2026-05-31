use crate::object::ObjectRef;

#[derive(Debug, Clone)]
pub struct ValueAssignState {
    pub needs_to_resolve_right_side: bool,
    pub right_value: Option<ObjectRef>,
    pub left_value: Option<ObjectRef>,
}
