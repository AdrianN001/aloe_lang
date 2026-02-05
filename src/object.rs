
pub mod integer;
pub mod boolean;
pub mod null;

use null::Null;
use boolean::Boolean;
use integer::Integer;

#[derive(Debug, PartialEq, Eq)]
pub enum Object{
    Int(Integer),
    Bool(Boolean), 
    Null(Null)
}

impl Object{

    pub const TRUE_BOOL_OBJECT: Object = Object::Bool(Boolean{
        value: true
    });
    pub const FALSE_BOOL_OBJECT: Object = Object::Bool(Boolean { 
        value: false 
    });
    pub const NULL_OBJECT: Object = Object::Null(Null{});



    pub fn get_native_boolean_object(value: bool) -> Self{
        if value {
            Self::TRUE_BOOL_OBJECT
        }else{
            Self::FALSE_BOOL_OBJECT
        }
    }

    pub fn get_type(&self) -> String{
        match self{
            Object::Int(obj) => obj.get_type(),
            Object::Bool(obj) => obj.get_type(),
            Object::Null(obj) => obj.get_type(),
        }
    }

    pub fn inspect(&self) -> String{
        match self{
            Object::Int(obj) => obj.inspect(),
            Object::Bool(obj) => obj.inspect(),
            Object::Null(obj) => obj.inspect(),
        }
    }
}



