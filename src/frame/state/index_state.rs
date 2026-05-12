use crate::object::ObjectRef;

#[derive(Debug, Clone)]
pub struct IndexState {
    pub indexable: Option<ObjectRef>,
    pub index: Option<ObjectRef>,
}
