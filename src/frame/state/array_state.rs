use crate::object::ObjectRef;

#[derive(Debug)]
pub struct ArrayState {
    pub elements: Vec<ObjectRef>,

    pub curr_element: usize,
    pub number_of_elements: usize,
}
