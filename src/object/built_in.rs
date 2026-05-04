mod array_method;
mod async_await;
mod console;
mod error;
mod io;
mod iterator;
mod len;
mod random;
mod time;
mod utils;

use crate::object::{
    ObjectRef,
    built_in::{
        array_method::{
            first_builtin_function, last_builtin_function, push_builtin_function,
            rest_builtin_function,
        },
        async_await::spawn_builtin_function,
        console::{
            console_read_builtin_function, console_write_builtin_function,
            console_writeln_builtin_function,
        },
        error::error_builtin_function,
        io::{async_tcp_bind_builtin_function, async_tcp_connect_builtin_function, open_builtin_function, path_builtin_function, tcp_bind_builtin_function, tcp_connect_builtin_function},
        iterator::range_builtin_function,
        len::len_builtin_function,
        random::random_builtin_function,
        time::{awaitable_sleep_builtin_function, sleep, time_builtin_function},
        utils::{inspect_builtin_function, type_builtin_function},
    },
    panic_obj::RuntimeSignal,
    stack_environment::EnvRef,
    state::StateRef,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BuiltIn {
    Len, // len(string), len(array)

    Rest,
    First,
    Last,
    Push,

    Print,
    Println,
    Read,

    Type,
    Inspect,

    Range,

    Random,
    Err,

    Open,
    Path,
    TCPBind,
    TCPConnect,
    ATCPBind,
    ATCPConnect,

    Sleep,
    Sleep2,
    Time,

    Spawn,
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
    ) -> Result<ObjectRef, RuntimeSignal> {
        match self {
            BuiltIn::Len => len_builtin_function(args, state),

            BuiltIn::Rest => rest_builtin_function(args, state),
            BuiltIn::First => first_builtin_function(args, state),
            BuiltIn::Last => last_builtin_function(args, state),
            BuiltIn::Push => push_builtin_function(args, state),

            BuiltIn::Print => Ok(console_write_builtin_function(args, environ)),
            BuiltIn::Println => Ok(console_writeln_builtin_function(args)),
            BuiltIn::Read => Ok(console_read_builtin_function()),

            BuiltIn::Type => type_builtin_function(args, state),
            BuiltIn::Inspect => inspect_builtin_function(args, state),

            BuiltIn::Range => range_builtin_function(args, state),
            BuiltIn::Random => Ok(random_builtin_function()),

            BuiltIn::Err => error_builtin_function(args, state),

            BuiltIn::Open => open_builtin_function(args, state),
            BuiltIn::Path => path_builtin_function(args, state),
            BuiltIn::TCPBind => tcp_bind_builtin_function(args, state),
            BuiltIn::TCPConnect => tcp_connect_builtin_function(args, state),
            BuiltIn::ATCPBind => async_tcp_bind_builtin_function(args, state),
            BuiltIn::ATCPConnect => async_tcp_connect_builtin_function(args, state),

            BuiltIn::Sleep => sleep(args, state),
            BuiltIn::Sleep2 => awaitable_sleep_builtin_function(args, state, environ),
            BuiltIn::Time => time_builtin_function(),

            BuiltIn::Spawn => spawn_builtin_function(args, state),
        }
    }
}
