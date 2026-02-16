

#[derive(PartialEq, Eq, Clone)]
pub struct StringObj{
    pub value: String
}


impl StringObj{
    pub fn get_type(&self) -> String{
        "string".into()
    }

    pub fn inspect(&self) -> String{
        self.value.to_string()
    }
}
