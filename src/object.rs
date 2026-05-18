pub mod array;
pub mod async_function;
pub mod boolean;
pub mod break_value;
pub mod built_in;
pub mod copy;
pub mod error;
pub mod float_obj;
pub mod function;
pub mod future;
pub mod hashable;
pub mod hashmap;
pub mod integer;
pub mod iterator;
pub mod member;
pub mod module;
pub mod native_object;
pub mod null;
pub mod operation;
pub mod panic_obj;
pub mod return_value;
pub mod stack_environment;
pub mod state;
pub mod string_obj;
pub mod struct_model;
pub mod struct_object;
pub mod truthy;

use std::{cell::RefCell, rc::Rc};

use array::Array;
use boolean::Boolean;
use built_in::BuiltIn;
use error::Error;
use float_obj::FloatObj;
use function::Function;
use hashmap::HashMap;
use integer::Integer;
use null::Null;
use return_value::ReturnValue;
use string_obj::StringObj;

use crate::object::{
    async_function::AsyncFunction, break_value::BreakValue, error::error_type::ErrorType,
    future::FutureObj, hashable::Hashable, hashmap::HashKey, iterator::Iterator,
    module::ModuleObject, native_object::NativeObject, state::StateRef, struct_model::StructModel,
    struct_object::StructObject,
};

pub type ObjectRef = Rc<RefCell<Object>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Object {
    // stack types
    Int(Integer),
    FloatObj(FloatObj),
    Bool(Boolean),

    Null(Null),
    BuiltIn(BuiltIn),
    ReturnVal(ReturnValue),
    BreakVal(BreakValue),
    Continue,

    // heap types
    String(Box<StringObj>),
    Iterator(Box<Iterator>),
    Func(Box<Function>),
    AsyncFunc(Box<AsyncFunction>),
    Err(Box<Error>),
    Array(Box<Array>),
    HashMap(Box<HashMap>),
    StructModel(Box<StructModel>),
    StructObject(Box<StructObject>),
    Native(Box<NativeObject>),
    Future(Box<FutureObj>),
    Module(Box<ModuleObject>),
}

impl Object {
    pub const TRUE_BOOL_OBJECT: Object = Object::Bool(Boolean { value: true });
    pub const FALSE_BOOL_OBJECT: Object = Object::Bool(Boolean { value: false });
    pub const NULL_OBJECT: Object = Object::Null(Null {});

    pub fn get_native_boolean_object(value: bool) -> Self {
        if value {
            Self::TRUE_BOOL_OBJECT
        } else {
            Self::FALSE_BOOL_OBJECT
        }
    }

    pub fn new_error(type_of: ErrorType, error_value: String, state: StateRef) -> Self {
        Object::Err(Box::new(Error {
            type_of,
            value: error_value,
            state: state.clone(),
        }))
    }

    pub fn get_type(&self) -> String {
        match self {
            Object::Int(obj) => obj.get_type(),
            Object::Bool(obj) => obj.get_type(),
            Object::Null(obj) => obj.get_type(),
            Object::Func(obj) => obj.get_type(),
            Object::ReturnVal(obj) => obj.get_type(),
            Object::String(obj) => obj.get_type(),
            Object::BuiltIn(obj) => obj.get_type(),
            Object::Err(obj) => obj.get_type(),
            Object::Array(obj) => obj.get_type(),
            Object::HashMap(obj) => obj.get_type(),
            Object::FloatObj(obj) => obj.get_type(),
            Object::Iterator(obj) => obj.get_type(),
            Object::BreakVal(obj) => obj.get_type(),
            Object::StructModel(obj) => obj.get_type(),
            Object::StructObject(obj) => obj.get_type(),
            Object::Continue => "continue".to_string(),
            Object::Native(native) => native.get_type(),
            Object::AsyncFunc(async_function) => async_function.get_type(),
            Object::Future(future) => future.get_type(),
            Object::Module(module) => module.get_type(),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Int(obj) => obj.inspect(),
            Object::Bool(obj) => obj.inspect(),
            Object::Null(obj) => obj.inspect(),
            Object::Func(function) => function.inspect(),
            Object::ReturnVal(obj) => obj.inspect(),
            Object::String(obj) => obj.inspect(),
            Object::BuiltIn(obj) => obj.inspect(),
            Object::Err(obj) => obj.inspect(),
            Object::Array(obj) => obj.inspect(),
            Object::HashMap(obj) => obj.inspect(),
            Object::FloatObj(obj) => obj.inspect(),
            Object::Iterator(obj) => obj.inspect(),
            Object::BreakVal(obj) => obj.inspect(),
            Object::StructModel(obj) => obj.inspect(),
            Object::StructObject(obj) => obj.inspect(),
            Object::Continue => "continue".to_string(),
            Object::Native(native) => native.inspect(),
            Object::AsyncFunc(async_function) => async_function.inspect(),
            Object::Future(future) => future.inspect(),
            Object::Module(module) => module.inspect(),
        }
    }

    pub fn hash(&self) -> Result<HashKey, String> {
        match self {
            Object::String(s) => Ok(s.hash()),
            Object::Int(i) => Ok(i.hash()),
            Object::Bool(b) => Ok(b.hash()),
            Object::FloatObj(f) => Ok(f.hash()),
            _ => Err(format!(
                "object with type: {} is not hashable",
                &self.get_type()
            )),
        }
    }
}

pub fn new_objectref(obj: Object) -> ObjectRef {
    Rc::new(RefCell::new(obj))
}
