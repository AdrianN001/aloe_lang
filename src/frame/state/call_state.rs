use crate::object::ObjectRef;

#[derive(Debug, Clone)]
pub struct CallState {
    pub parameters: Vec<ObjectRef>,

    pub current_argument: usize,
    pub parameters_required_by_func: usize,
}
