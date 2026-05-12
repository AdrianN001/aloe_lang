use crate::object::ObjectRef;

#[derive(Debug, Clone)]
pub struct WhileState {
    pub is_infinite: bool,
    pub conditional_value: Option<ObjectRef>,
    pub is_head_ready: bool,
}
