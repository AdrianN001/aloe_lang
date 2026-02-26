mod array_method;
mod console;
mod len;
mod utils;

use crate::object::{
    ObjectRef,
    built_in::{
        array_method::{
            first_builtin_function, last_builtin_function, push_builtin_function,
            rest_builtin_function,
        },
        console::console_write_builtin_function,
        len::len_builtin_function,
        utils::{inspect_builtin_function, type_builtin_function},
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
            Self::Len => len_builtin_function(args),

            Self::Rest => rest_builtin_function(args),
            Self::First => first_builtin_function(args),
            Self::Last => last_builtin_function(args),
            Self::Push => push_builtin_function(args),

            Self::Print => console_write_builtin_function(args),

            Self::Type => type_builtin_function(args),
            Self::Inspect => inspect_builtin_function(args),
        }
    }
}
