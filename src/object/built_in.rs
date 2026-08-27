pub mod async_await;
mod console;
mod error;
mod iterator;
mod len;
mod math;
mod memory;
mod native;
mod os;
mod process;
mod random;
mod test;
mod time;
mod utils;

use crate::object::{
    ObjectRef,
    built_in::{
        async_await::spawn_builtin_function,
        console::{
            console_read_async_builtin_function, console_read_builtin_function,
            console_write_builtin_function, console_writeln_builtin_function,
        },
        error::{error_builtin_function, panic_buitin_function},
        iterator::range_builtin_function,
        len::len_builtin_function,
        memory::{
            id_builtin_function, number_of_references_builtin_function, size_of_builtin_function,
        },
        os::{
            arch_builtin_function, get_current_dir_builtin_function, get_env_builtin_function,
            get_home_dir_builtin_function, get_temp_dir_builtin_function,
            platform_builtin_function, set_env_builtin_function, unset_env_builtin_function,
        },
        process::{args_builtin_function, exit_builtin_function, pid_builtin_function},
        random::random_builtin_function,
        time::{awaitable_sleep_builtin_function, sleep, time_builtin_function},
        utils::{inspect_builtin_function, line_number_builtin_function, type_builtin_function},
    },
    panic_obj::RuntimeSignal,
    stack_environment::EnvRef,
    state::StateRef,
};

#[derive(Clone, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
pub enum BuiltIn {
    // preludes
    P_LEN,
    P_PRINT,
    P_PRINTLN,
    P_READ,

    P_TYPE,
    P_INSPECT,
    P_LINE,

    P_RANGE,

    P_ERROR,
    P_PANIC,

    P_ASSERT,

    // loaded into _io module
    U_IO_READ_ASYNC,

    // loaded into random module
    RANDOM_RND,

    // loaded into _sys module
    U_SYS_EXIT,
    U_SYS_PID,
    U_SYS_ARGS,

    // loaded into _time module
    U_TIME_SLEEP,
    U_TIME_SLEEP_ASYNC,
    U_TIME_TIME,

    // loaded into _async module
    U_ASYNC_SPAWN,

    // loaded into _memory module
    U_MEMORY_ID,
    U_MEMORY_REF_COUNT,
    U_MEMORY_SIZE,

    //loaded into math module
    MATH_CEIL,
    MATH_FLOOR,
    MATH_TRUNC,
    MATH_FABS,
    MATH_SQRT,
    MATH_EXP,
    MATH_EXPM1,
    MATH_LOG,
    MATH_LOG10,
    MATH_LOG1P,
    MATH_POW,
    MATH_FMOD,
    MATH_HYPOT,
    MATH_COPYSIGN,
    MATH_DEGREES,
    MATH_RADIANS,
    MATH_SIN,
    MATH_COS,
    MATH_TAN,
    MATH_ASIN,
    MATH_ACOS,
    MATH_ATAN,
    MATH_ATAN2,
    MATH_SINH,
    MATH_COSH,
    MATH_TANH,
    MATH_ASINH,
    MATH_ACOSH,
    MATH_ATANH,
    MATH_LDEXP,
    MATH_FREXP,
    MATH_MODF,
    MATH_GCD,
    MATH_LCM,
    MATH_FACTORIAL,

    // loaded into _os module
    U_OS_GET_ENV,
    U_OS_SET_ENV,
    U_OS_UNSET_ENV,
    U_OS_CURRENT_DIR,
    U_OS_HOME_DIR,
    U_OS_TEMP_DIR,
    U_OS_PLATFORM,
    U_OS_ARCH,

    // loaded into _ntv module
    U_NTV_SPAWN,
}

impl BuiltIn {
    pub fn get_type(&self) -> String {
        "<type built-in>".into()
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
            BuiltIn::P_LEN => len_builtin_function(args, state),

            BuiltIn::P_PRINT => Ok(console_write_builtin_function(args, environ)),
            BuiltIn::P_PRINTLN => Ok(console_writeln_builtin_function(args)),
            BuiltIn::P_READ => Ok(console_read_builtin_function()),
            BuiltIn::U_IO_READ_ASYNC => Ok(console_read_async_builtin_function()),

            BuiltIn::P_TYPE => type_builtin_function(args, state),
            BuiltIn::P_INSPECT => inspect_builtin_function(args, state),
            BuiltIn::P_LINE => line_number_builtin_function(args, state),

            BuiltIn::P_RANGE => range_builtin_function(args, state),
            BuiltIn::RANDOM_RND => Ok(random_builtin_function()),

            BuiltIn::P_ERROR => error_builtin_function(args, state),
            BuiltIn::P_PANIC => panic_buitin_function(args, state),

            BuiltIn::U_SYS_EXIT => exit_builtin_function(args, state),
            BuiltIn::U_SYS_ARGS => args_builtin_function(args, state),
            BuiltIn::U_SYS_PID => pid_builtin_function(args, state),

            BuiltIn::U_TIME_SLEEP => sleep(args, state),
            BuiltIn::U_TIME_SLEEP_ASYNC => awaitable_sleep_builtin_function(args, state, environ),
            BuiltIn::U_TIME_TIME => time_builtin_function(),

            BuiltIn::U_ASYNC_SPAWN => spawn_builtin_function(args, state),

            BuiltIn::U_MEMORY_ID => id_builtin_function(args, state),
            BuiltIn::U_MEMORY_REF_COUNT => number_of_references_builtin_function(args, state),
            BuiltIn::U_MEMORY_SIZE => size_of_builtin_function(args, state),

            BuiltIn::MATH_CEIL => math::ceil_builtin_function(args, state),
            BuiltIn::MATH_FLOOR => math::floor_builtin_function(args, state),
            BuiltIn::MATH_TRUNC => math::trunc_builtin_function(args, state),
            BuiltIn::MATH_FABS => math::fabs_builtin_function(args, state),
            BuiltIn::MATH_SQRT => math::sqrt_builtin_function(args, state),
            BuiltIn::MATH_EXP => math::exp_builtin_function(args, state),
            BuiltIn::MATH_EXPM1 => math::expm1_builtin_function(args, state),
            BuiltIn::MATH_LOG => math::log_builtin_function(args, state),
            BuiltIn::MATH_LOG10 => math::log10_builtin_function(args, state),
            BuiltIn::MATH_LOG1P => math::log1p_builtin_function(args, state),
            BuiltIn::MATH_POW => math::pow_builtin_function(args, state),
            BuiltIn::MATH_FMOD => math::fmod_builtin_function(args, state),
            BuiltIn::MATH_HYPOT => math::hypot_builtin_function(args, state),
            BuiltIn::MATH_COPYSIGN => math::copysign_builtin_function(args, state),
            BuiltIn::MATH_DEGREES => math::degrees_builtin_function(args, state),
            BuiltIn::MATH_RADIANS => math::radians_builtin_function(args, state),
            BuiltIn::MATH_SIN => math::sin_builtin_function(args, state),
            BuiltIn::MATH_COS => math::cos_builtin_function(args, state),
            BuiltIn::MATH_TAN => math::tan_builtin_function(args, state),
            BuiltIn::MATH_ASIN => math::asin_builtin_function(args, state),
            BuiltIn::MATH_ACOS => math::acos_builtin_function(args, state),
            BuiltIn::MATH_ATAN => math::atan_builtin_function(args, state),
            BuiltIn::MATH_ATAN2 => math::atan2_builtin_function(args, state),
            BuiltIn::MATH_SINH => math::sinh_builtin_function(args, state),
            BuiltIn::MATH_COSH => math::cosh_builtin_function(args, state),
            BuiltIn::MATH_TANH => math::tanh_builtin_function(args, state),
            BuiltIn::MATH_ASINH => math::asinh_builtin_function(args, state),
            BuiltIn::MATH_ACOSH => math::acosh_builtin_function(args, state),
            BuiltIn::MATH_ATANH => math::atanh_builtin_function(args, state),
            BuiltIn::MATH_LDEXP => math::ldexp_builtin_function(args, state),
            BuiltIn::MATH_FREXP => math::frexp_builtin_function(args, state),
            BuiltIn::MATH_MODF => math::modf_builtin_function(args, state),
            BuiltIn::MATH_GCD => math::gcd_builtin_function(args, state),
            BuiltIn::MATH_LCM => math::lcm_builtin_function(args, state),
            BuiltIn::MATH_FACTORIAL => math::factorial_builtin_function(args, state),

            BuiltIn::P_ASSERT => test::assert_builtin_function(args, state),

            //OS
            BuiltIn::U_OS_GET_ENV => get_env_builtin_function(args, state),
            BuiltIn::U_OS_SET_ENV => set_env_builtin_function(args, state),
            BuiltIn::U_OS_UNSET_ENV => unset_env_builtin_function(args, state),
            BuiltIn::U_OS_CURRENT_DIR => get_current_dir_builtin_function(args, state),
            BuiltIn::U_OS_HOME_DIR => get_home_dir_builtin_function(args, state),
            BuiltIn::U_OS_TEMP_DIR => get_temp_dir_builtin_function(args, state),
            BuiltIn::U_OS_PLATFORM => platform_builtin_function(args, state),
            BuiltIn::U_OS_ARCH => arch_builtin_function(args, state),

            //Native
            BuiltIn::U_NTV_SPAWN => native::spawn_native_builtin_function(args, environ, state),
        }
    }
}
