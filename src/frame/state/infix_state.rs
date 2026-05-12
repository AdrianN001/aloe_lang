use crate::object::ObjectRef;

#[derive(Debug, Clone)]
pub struct InfixState {
    pub left: Option<ObjectRef>,
    pub right: Option<ObjectRef>,
}
