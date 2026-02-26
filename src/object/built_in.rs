mod array_method;
mod console;
mod len;
mod utils;
mod iterator;

use crate::object::{
    ObjectRef,
    built_in::{
        array_method::{
            first_builtin_function, last_builtin_function, push_builtin_function,
            rest_builtin_function,
        }, console::console_write_builtin_function, iterator::range_builtin_function, len::len_builtin_function, utils::{inspect_builtin_function, type_builtin_function}
    },
};

#[derive(Clone, PartialEq, Eq)]
pub enum BuiltIn {
    Len, // len(string), len(array)

    Rest,
    First,
    Last,
    Push,

    Print,

    Type,
    Inspect,

    Range
}

impl BuiltIn {
    pub fn get_type(&self) -> String {
        "built-in function".into()
    }

    pub fn inspect(&self) -> String {
        "built-in function".into()
    }

    pub fn call(&self, args: &[ObjectRef]) -> ObjectRef {
        match self {
            BuiltIn::Len => len_builtin_function(args),

            BuiltIn::Rest => rest_builtin_function(args),
            BuiltIn::First => first_builtin_function(args),
            BuiltIn::Last => last_builtin_function(args),
            BuiltIn::Push => push_builtin_function(args),

            BuiltIn::Print => console_write_builtin_function(args),

            BuiltIn::Type => type_builtin_function(args),
            BuiltIn::Inspect => inspect_builtin_function(args),

            BuiltIn::Range => range_builtin_function(args),
            
        }
    }
}
