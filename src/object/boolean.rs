
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Boolean{
    pub value: bool
}

impl Boolean{
    pub fn get_type(&self) -> String{
        "boolean".into()
    }

    pub fn inspect(&self) -> String{
        self.value.to_string()
    }
}


