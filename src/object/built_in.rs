mod len;

use crate::object::{Object, built_in::len::{len_builtin_function}};



#[derive(Clone, PartialEq, Eq)]
pub enum BuiltIn{
    Len
}


impl BuiltIn{
    pub fn get_type(&self) -> String{
        "built-in function".into()
    }

    pub fn inspect(&self) -> String{
        "built-in function".into()
    }

   
    pub fn call(&self, args: &[Object]) -> Object{
        match self{
            Self::Len => len_builtin_function(args)
        }
    }
}
