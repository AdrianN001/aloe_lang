use crate::object::Object;



#[derive(PartialEq, Eq, Clone)]
pub struct ReturnValue{
    pub value: Box<Object>
}

impl ReturnValue{
    pub fn get_type(&self) -> String{
        "return value".into()
    }

    pub fn inspect(&self) -> String{
        self.value.inspect()
    }
}
