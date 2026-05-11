use crate::object::ObjectRef;

#[derive(Debug)]
pub struct InfixState {
    pub left: Option<ObjectRef>,
    pub right: Option<ObjectRef>,
}
