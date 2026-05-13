use crate::object::{ObjectRef, iterator::Iterator};

#[derive(Debug, Clone)]
pub struct ForState {
    pub is_infinite: bool,
    pub provided_object: Option<ObjectRef>,
    pub iterator: Option<Iterator>,
    pub iteration_variable_name: Option<String>,
}
