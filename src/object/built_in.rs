
mod array_method;
mod len;


use crate::object::{Object, built_in::{array_method::{first_builtin_function, last_builtin_function, push_builtin_function, rest_builtin_function}, len::len_builtin_function}};



#[derive(Clone, PartialEq, Eq)]
pub enum BuiltIn{
    Len,            // len(string), len(array)

    Rest,
    First,
    Last, 
    Push, 
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
            Self::Len => len_builtin_function(args),
            
            Self::Rest =>rest_builtin_function(args),
            Self::First=>first_builtin_function(args),
            Self::Last =>last_builtin_function(args),
            Self::Push =>push_builtin_function(args)
        }
    }
}
