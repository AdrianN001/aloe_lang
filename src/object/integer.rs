

#[derive(PartialEq, Eq,Clone, Debug)]
pub struct Integer{
    pub value: i64
}

impl Integer{
    pub fn get_type(&self) -> String{
        "integer".into()
    }

    pub fn inspect(&self) -> String{
        self.value.to_string()
    }
}
