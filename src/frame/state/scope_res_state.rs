use crate::object::ObjectRef;

#[derive(Debug, Clone)]
pub struct ScopeResolutionState {
    pub left_side: Option<ObjectRef>,
    pub call_buffer: Vec<ObjectRef>,
}
