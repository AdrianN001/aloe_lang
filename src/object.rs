
pub mod integer;
pub mod boolean;
pub mod null;
pub mod return_value;
pub mod function;
pub mod truthy;
pub mod stack_environment;
pub mod string_obj;
pub mod built_in;
pub mod error;
pub mod array;


use array::Array;
use error::Error;
use built_in::BuiltIn;
use string_obj::StringObj;
use return_value::ReturnValue;
use null::Null;
use boolean::Boolean;
use integer::Integer;
use function::Function;

#[derive(PartialEq, Eq, Clone)]
pub enum Object{
    Int(Integer),
    Bool(Boolean),
    String(StringObj),

    BuiltIn(BuiltIn),

    Func(Function),
    ReturnVal(ReturnValue),
    Err(Error),
    Array(Array),

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

    pub fn new_error(error_value: String) -> Self{
        Self::Err(Error { value: error_value })
    }

    pub fn get_type(&self) -> String{
        match self{
            Object::Int(obj) => obj.get_type(),
            Object::Bool(obj) => obj.get_type(),
            Object::Null(obj) => obj.get_type(),
            Object::Func(obj) => obj.get_type(),
            Object::ReturnVal(obj) => obj.get_type(),
            Object::String(obj) => obj.get_type(),
            Object::BuiltIn(obj) => obj.get_type(),
            Object::Err(obj) => obj.get_type(),
            Object::Array(obj) => obj.get_type(),
        }
    }

    pub fn inspect(&self) -> String{
        match self{
            Object::Int(obj) => obj.inspect(),
            Object::Bool(obj) => obj.inspect(),
            Object::Null(obj) => obj.inspect(),
            Object::Func(function) => function.inspect(),
            Object::ReturnVal(obj) => obj.inspect(),
            Object::String(obj) => obj.inspect(),
            Object::BuiltIn(obj) => obj.inspect(),
            Object::Err(obj) => obj.inspect(),
            Object::Array(obj) => obj.inspect(),
        }
    }
}



