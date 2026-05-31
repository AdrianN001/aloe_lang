use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    native_object::{NativeObject, process::CommandWrapper},
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

pub fn command_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 argument for __cmd(), got: {}", args.len()),
            state,
        )));
    }

    let program = match &*args[0].borrow() {
        Object::String(program_str) => program_str.value.clone(),
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "expected str as the first argument for __cmd(), got: {}",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let cmd_wrapper = CommandWrapper::new(&program);

    Ok(new_objectref(Object::Native(Box::new(
        NativeObject::Command(cmd_wrapper),
    ))))
}
