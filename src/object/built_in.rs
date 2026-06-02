mod array_method;
pub mod async_await;
mod console;
mod error;
mod io;
mod iterator;
mod len;
mod math;
mod memory;
mod process;
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
            console_read_async_builtin_function, console_read_builtin_function,
            console_write_builtin_function, console_writeln_builtin_function,
        },
        error::{error_builtin_function, panic_buitin_function},
        io::{
            async_tcp_bind_builtin_function, async_tcp_connect_builtin_function,
            async_udp_bind_builtin_function, open_builtin_function, path_builtin_function,
            tcp_bind_builtin_function, tcp_connect_builtin_function, udp_bind_builtin_function,
        },
        iterator::range_builtin_function,
        len::len_builtin_function,
        memory::{
            id_builtin_function, number_of_references_builtin_function, size_of_builtin_function,
        },
        process::command_builtin_function,
        random::random_builtin_function,
        time::{awaitable_sleep_builtin_function, sleep, time_builtin_function},
        utils::{inspect_builtin_function, line_number_builtin_function, type_builtin_function},
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
    ARead,

    Type,
    Inspect,
    Line,

    Range,

    Random,
    Err,
    Panic,

    Open,
    Path,
    TCPBind,
    TCPConnect,
    ATCPBind,
    ATCPConnect,
    AUDPBind,
    UDPBind,

    Cmd,

    Sleep,
    Sleep2,
    Time,

    Spawn,

    Id,
    RefNumber,
    Size,

    Ceil,
    Floor,
    Trunc,
    Fabs,
    Sqrt,
    Exp,
    Expm1,
    Log,
    Log10,
    Log1p,
    Pow,
    Fmod,
    Hypot,
    Copysign,
    Degrees,
    Radians,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Atan2,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Ldexp,
    Frexp,
    Modf,
    Gcd,
    Lcm,
    Factorial,
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
            BuiltIn::ARead => Ok(console_read_async_builtin_function()),

            BuiltIn::Type => type_builtin_function(args, state),
            BuiltIn::Inspect => inspect_builtin_function(args, state),
            BuiltIn::Line => line_number_builtin_function(args, state),

            BuiltIn::Range => range_builtin_function(args, state),
            BuiltIn::Random => Ok(random_builtin_function()),

            BuiltIn::Err => error_builtin_function(args, state),
            BuiltIn::Panic => panic_buitin_function(args, state),

            BuiltIn::Open => open_builtin_function(args, state),
            BuiltIn::Path => path_builtin_function(args, state),
            BuiltIn::TCPBind => tcp_bind_builtin_function(args, state),
            BuiltIn::TCPConnect => tcp_connect_builtin_function(args, state),
            BuiltIn::ATCPBind => async_tcp_bind_builtin_function(args, state),
            BuiltIn::ATCPConnect => async_tcp_connect_builtin_function(args, state),
            BuiltIn::AUDPBind => async_udp_bind_builtin_function(args, state),
            BuiltIn::UDPBind => udp_bind_builtin_function(args, state),

            BuiltIn::Cmd => command_builtin_function(args, state),

            BuiltIn::Sleep => sleep(args, state),
            BuiltIn::Sleep2 => awaitable_sleep_builtin_function(args, state, environ),
            BuiltIn::Time => time_builtin_function(),

            BuiltIn::Spawn => spawn_builtin_function(args, state),

            BuiltIn::Id => id_builtin_function(args, state),
            BuiltIn::RefNumber => number_of_references_builtin_function(args, state),
            BuiltIn::Size => size_of_builtin_function(args, state),

            BuiltIn::Ceil => math::ceil_builtin_function(args, state),
            BuiltIn::Floor => math::floor_builtin_function(args, state),
            BuiltIn::Trunc => math::trunc_builtin_function(args, state),
            BuiltIn::Fabs => math::fabs_builtin_function(args, state),
            BuiltIn::Sqrt => math::sqrt_builtin_function(args, state),
            BuiltIn::Exp => math::exp_builtin_function(args, state),
            BuiltIn::Expm1 => math::expm1_builtin_function(args, state),
            BuiltIn::Log => math::log_builtin_function(args, state),
            BuiltIn::Log10 => math::log10_builtin_function(args, state),
            BuiltIn::Log1p => math::log1p_builtin_function(args, state),
            BuiltIn::Pow => math::pow_builtin_function(args, state),
            BuiltIn::Fmod => math::fmod_builtin_function(args, state),
            BuiltIn::Hypot => math::hypot_builtin_function(args, state),
            BuiltIn::Copysign => math::copysign_builtin_function(args, state),
            BuiltIn::Degrees => math::degrees_builtin_function(args, state),
            BuiltIn::Radians => math::radians_builtin_function(args, state),
            BuiltIn::Sin => math::sin_builtin_function(args, state),
            BuiltIn::Cos => math::cos_builtin_function(args, state),
            BuiltIn::Tan => math::tan_builtin_function(args, state),
            BuiltIn::Asin => math::asin_builtin_function(args, state),
            BuiltIn::Acos => math::acos_builtin_function(args, state),
            BuiltIn::Atan => math::atan_builtin_function(args, state),
            BuiltIn::Atan2 => math::atan2_builtin_function(args, state),
            BuiltIn::Sinh => math::sinh_builtin_function(args, state),
            BuiltIn::Cosh => math::cosh_builtin_function(args, state),
            BuiltIn::Tanh => math::tanh_builtin_function(args, state),
            BuiltIn::Asinh => math::asinh_builtin_function(args, state),
            BuiltIn::Acosh => math::acosh_builtin_function(args, state),
            BuiltIn::Atanh => math::atanh_builtin_function(args, state),
            BuiltIn::Ldexp => math::ldexp_builtin_function(args, state),
            BuiltIn::Frexp => math::frexp_builtin_function(args, state),
            BuiltIn::Modf => math::modf_builtin_function(args, state),
            BuiltIn::Gcd => math::gcd_builtin_function(args, state),
            BuiltIn::Lcm => math::lcm_builtin_function(args, state),
            BuiltIn::Factorial => math::factorial_builtin_function(args, state),
        }
    }
}
