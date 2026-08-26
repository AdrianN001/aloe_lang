use crate::object::{
    Object, ObjectRef,
    error::{error_type::ErrorType, panic_type::PanicType},
    native_object::{NativeObject, path::PathWrapper, spawn::Spawnable},
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

impl Spawnable for PathWrapper {
    fn spawn(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        if args.len() != 1 {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected number of parameter for __path(). Expected: 1, got: '{}'",
                    args.len()
                ),
                state,
            )));
        }

        let arg_borrow = args[0].borrow();

        let path_arg = match &*arg_borrow {
            Object::String(str_obj) => &str_obj.value,
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "unexpected parameter type for __path(). Expected: 'str', got: '{}'",
                        other_type.get_type()
                    ),
                    state,
                )));
            }
        };

        let wrapper = match PathWrapper::new(path_arg) {
            Ok(wrapper) => wrapper,
            Err(err_feedback) => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::PathResolve,
                    err_feedback.to_string(),
                    state,
                )));
            }
        };

        Ok(new_objectref(Object::Native(Box::new(NativeObject::Path(
            wrapper,
        )))))
    }
}
