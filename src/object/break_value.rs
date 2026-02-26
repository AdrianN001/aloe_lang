use crate::object::ObjectRef;

#[derive(PartialEq, Eq, Clone)]
pub struct BreakValue {
    pub value: Box<ObjectRef>,
}

impl BreakValue {
    pub fn get_type(&self) -> String {
        "break value".into()
    }

    pub fn inspect(&self) -> String {
        self.value.borrow().inspect()
    }

    pub fn unwrap_to_value(&self) -> ObjectRef {
        *self.value.clone()
    }
}
