use crate::object::ObjectRef;

//TODO: Diese ganze mist kann mit Runtime::Break ersetzt werden. Spater
#[derive(PartialEq, Eq, Clone, Debug)]
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
