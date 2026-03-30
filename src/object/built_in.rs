mod array_method;
mod console;
mod error;
mod io;
mod iterator;
mod len;
mod random;
mod utils;

use crate::object::{
    ObjectRef,
    built_in::{
        array_method::{
            first_builtin_function, last_builtin_function, push_builtin_function,
            rest_builtin_function,
        },
        console::{console_read_builtin_function, console_write_builtin_function},
        error::error_builtin_function,
        io::open_builtin_function,
        iterator::range_builtin_function,
        len::len_builtin_function,
        random::random_builtin_function,
        utils::{inspect_builtin_function, type_builtin_function},
    },
    panic_obj::PanicObj,
    stack_environment::EnvRef,
    state::StateRef,
};

#[derive(Clone, PartialEq, Eq)]
pub enum BuiltIn {
    Len, // len(string), len(array)

    Rest,
    First,
    Last,
    Push,

    Print,
    Read,

    Type,
    Inspect,

    Range,

    Random,
    Err,

    Open,
}

impl BuiltIn {
    pub fn get_type(&self) -> String {
        "built-in function".into()
    }

    pub fn inspect(&self) -> String {
        "built-in function".into()
    }

    pub fn call(
        &self,
        args: &[ObjectRef],
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match self {
            BuiltIn::Len => len_builtin_function(args, state),

            BuiltIn::Rest => rest_builtin_function(args, state),
            BuiltIn::First => first_builtin_function(args, state),
            BuiltIn::Last => last_builtin_function(args, state),
            BuiltIn::Push => push_builtin_function(args, state),

            BuiltIn::Print => Ok(console_write_builtin_function(args, environ)),
            BuiltIn::Read => Ok(console_read_builtin_function()),

            BuiltIn::Type => type_builtin_function(args, state),
            BuiltIn::Inspect => inspect_builtin_function(args, state),

            BuiltIn::Range => range_builtin_function(args, state),
            BuiltIn::Random => Ok(random_builtin_function()),

            BuiltIn::Err => error_builtin_function(args, state),

            BuiltIn::Open => open_builtin_function(args, state),
        }
    }
}
