use crate::object::ObjectRef;

#[derive(PartialEq, Eq, Clone)]
pub struct ReturnValue {
    pub value: Box<ObjectRef>,
}

impl ReturnValue {
    pub fn get_type(&self) -> String {
        "return value".into()
    }

    pub fn inspect(&self) -> String {
        self.value.borrow().inspect()
    }

    pub fn unwrap_to_value(&self) -> ObjectRef {
        *self.value.clone()
    }
}
