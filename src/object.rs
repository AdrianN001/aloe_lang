
pub mod integer;
pub mod boolean;
pub mod null;

use null::Null;
use boolean::Boolean;
use integer::Integer;

pub enum Object{
    Int(Integer),
    Bool(Boolean), 
    Null(Null)
}

impl Object{
    pub fn get_type(&self) -> String{
        match self{
            Object::Int(obj) => obj.get_type(),
            Object::Bool(obj) => obj.get_type(),
            Object::Null(obj) => obj.get_type(),
            _ => panic!()
        }
    }

    pub fn inspect(&self) -> String{
        match self{
            Object::Int(obj) => obj.inspect(),
            Object::Bool(obj) => obj.inspect(),
            Object::Null(obj) => obj.inspect(),
            _ => panic!()
        }
    }
}
