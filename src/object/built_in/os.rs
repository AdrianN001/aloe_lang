use crate::object::{
    Object, ObjectRef,
    error::{error_type::ErrorType, panic_type::PanicType},
    native_object::{NativeObject, path::PathWrapper},
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
    string_obj::StringObj,
};

pub fn get_env_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("get_env() takes exactly 1 argument ({} given)", args.len()),
            state,
        )));
    }

    let key_borrow = args[0].borrow();
    let key_str = match &*key_borrow {
        Object::String(s) => &s.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "get_env() argument must be a string ({} given)",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    match std::env::var(&key_str) {
        Ok(value) => Ok(new_objectref(Object::String(Box::new(StringObj { value })))),
        Err(err) => Ok(new_objectref(Object::new_error(
            ErrorType::EnvironmentVariableNotFound,
            err.to_string(),
            state,
        ))),
    }
}

pub fn set_env_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("set_env() takes exactly 2 arguments ({} given)", args.len()),
            state,
        )));
    }

    let key_borrow = args[0].borrow();
    let key_str = match &*key_borrow {
        Object::String(s) => &s.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "set_env() first argument must be a string ({} given)",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let value_borrow = args[1].borrow();
    let value_str = match &*value_borrow {
        Object::String(s) => &s.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "set_env() second argument must a string ({} given)",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    unsafe { std::env::set_var(&key_str, &value_str) };

    Ok(new_objectref(Object::NULL_OBJECT))
}

pub fn unset_env_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!(
                "unset_env() takes exactly 1 argument ({} given)",
                args.len()
            ),
            state,
        )));
    }

    let key_borrow = args[0].borrow();
    let key_str = match &*key_borrow {
        Object::String(s) => &s.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unset_env() argument must be a string ({} given)",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    unsafe {
        std::env::remove_var(&key_str);
    }

    Ok(new_objectref(Object::NULL_OBJECT))
}

pub fn get_current_dir_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if !args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!(
                "get_current_dir() takes no arguments ({} given)",
                args.len()
            ),
            state,
        )));
    }

    match std::env::current_dir() {
        Ok(path_buf) => {
            let path_wrapper = PathWrapper::new_from_pathbuf(path_buf);

            Ok(new_objectref(Object::Native(Box::new(NativeObject::Path(
                path_wrapper,
            )))))
        }
        Err(err) => Ok(new_objectref(Object::new_error(
            ErrorType::OS,
            err.to_string(),
            state,
        ))),
    }
}

pub fn get_home_dir_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if !args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("get_home_dir() takes no arguments ({} given)", args.len()),
            state,
        )));
    }

    match std::env::home_dir() {
        Some(path_buf) => {
            let path_wrapper = PathWrapper::new_from_pathbuf(path_buf);

            Ok(new_objectref(Object::Native(Box::new(NativeObject::Path(
                path_wrapper,
            )))))
        }
        None => Ok(new_objectref(Object::new_error(
            ErrorType::OS,
            "Home directory not found".to_string(),
            state,
        ))),
    }
}

pub fn get_temp_dir_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if !args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("get_temp_dir() takes no arguments ({} given)", args.len()),
            state,
        )));
    }

    let temp_dir_path_buf = std::env::temp_dir();
    let path_wrapper = PathWrapper::new_from_pathbuf(temp_dir_path_buf);

    Ok(new_objectref(Object::Native(Box::new(NativeObject::Path(
        path_wrapper,
    )))))
}

pub fn platform_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if !args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("platform() takes no arguments ({} given)", args.len()),
            state,
        )));
    }

    let platform_str = std::env::consts::OS.to_string();
    Ok(new_objectref(Object::String(Box::new(StringObj {
        value: platform_str,
    }))))
}

pub fn arch_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if !args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("arch() takes no arguments ({} given)", args.len()),
            state,
        )));
    }

    let arch_str = std::env::consts::ARCH.to_string();
    Ok(new_objectref(Object::String(Box::new(StringObj {
        value: arch_str,
    }))))
}
