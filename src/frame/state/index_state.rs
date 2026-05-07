use crate::object::ObjectRef;

#[derive(Debug)]
pub struct IndexState {
    pub indexable: Option<ObjectRef>,
    pub index: Option<ObjectRef>,
}
