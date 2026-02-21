

#[derive(PartialEq, Eq, Clone)]
pub struct Error{
    pub value: String
}


impl Error{

    pub fn get_type(&self) -> String{
        "error".into()
    }

    pub fn inspect(&self) -> String{
        self.value.clone()
    }
}
