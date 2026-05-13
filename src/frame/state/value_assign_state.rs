use crate::object::ObjectRef;

#[derive(Debug, Clone)]
pub struct ValueAssignState {
    pub right_value: Option<ObjectRef>,
    pub left_value: Option<ObjectRef>,
}
