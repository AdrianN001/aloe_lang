use crate::object::{
    Object, ObjectRef,
    array::Array,
    error::panic_type::PanicType,
    integer::Integer,
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
    string_obj::StringObj,
};

pub fn exit_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 argument for exit(), got: {}", args.len()),
            state,
        )));
    }

    let exit_code = match &*args[0].borrow() {
        Object::Int(num) => num.value as i32,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "expected int as the first argument for exit(), got: {}",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    std::process::exit(exit_code);
}

pub fn pid_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if !args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("pid() takes no arguments ({} given)", args.len()),
            state,
        )));
    }

    let pid = std::process::id();
    Ok(new_objectref(Object::Int(Integer { value: pid as i64 })))
}

pub fn args_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if !args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("args() takes no arguments ({} given)", args.len()),
            state,
        )));
    }

    let cmd_args: Vec<ObjectRef> = std::env::args()
        .map(|arg| new_objectref(Object::String(Box::new(StringObj { value: arg }))))
        .collect();

    Ok(new_objectref(Object::Array(Box::new(Array {
        items: cmd_args,
    }))))
}
