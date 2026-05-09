use crate::object::ObjectRef;

#[derive(Debug)]
pub struct HashMapState {
    pub current_element: usize,
    pub keys: Vec<ObjectRef>,
    pub values: Vec<ObjectRef>,
}
