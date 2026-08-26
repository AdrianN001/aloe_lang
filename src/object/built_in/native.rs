use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    native_object::spawn::generate_native_object,
    panic_obj::{PanicObj, RuntimeSignal},
    stack_environment::EnvRef,
    state::StateRef,
};

pub fn spawn_native_builtin_function(
    args: &[ObjectRef],
    _environ: EnvRef,
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected a name for __ntv, got: {} argument(s)", args.len()),
            state,
        )));
    }

    let name = match &*args[0].borrow() {
        Object::String(name) => name.value.clone(),
        _ => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!("expected a string for __ntv, got: {:?}", args[0]),
                state.clone(),
            )));
        }
    };

    let args_for_native = &args[1..];

    generate_native_object(&name, args_for_native, state)
}
