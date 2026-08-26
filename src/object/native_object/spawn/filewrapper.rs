use crate::object::{
    Object, ObjectRef,
    error::{error_type::ErrorType, panic_type::PanicType},
    native_object::{NativeObject, file::FileWrapper, spawn::Spawnable},
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

impl Spawnable for FileWrapper {
    fn spawn(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        match args.len() {
            1 => {
                // open("file_name");
                let file_name_borrow = args[0].borrow();

                let file_name_raw = match &*file_name_borrow {
                    Object::String(str) => str.value.clone(),
                    other_type => {
                        return Err(RuntimeSignal::Panic(PanicObj::new(
                            PanicType::WrongArgumentType,
                            format!(
                                "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                                other_type.get_type()
                            ),
                            state,
                        )));
                    }
                };

                let wrapper = match FileWrapper::new(file_name_raw) {
                    Ok(wrapper) => wrapper,
                    Err(err_feedback) => {
                        return Ok(new_objectref(Object::new_error(
                            ErrorType::FileOpening,
                            err_feedback,
                            state.clone(),
                        )));
                    }
                };

                Ok(new_objectref(Object::Native(Box::new(NativeObject::File(
                    Box::new(wrapper),
                )))))
            }

            2 => {
                // open("file_name", should_create);
                let file_name_borrow = args[0].borrow();
                let should_create_borrow = args[1].borrow();

                let file_name_raw = match &*file_name_borrow {
                    Object::String(str) => str.value.clone(),
                    other_type => {
                        return Err(RuntimeSignal::Panic(PanicObj::new(
                            PanicType::WrongArgumentType,
                            format!(
                                "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                                other_type.get_type()
                            ),
                            state,
                        )));
                    }
                };

                let should_crate_raw = match &*should_create_borrow {
                    Object::Bool(bool) => bool.value,
                    other_type => {
                        return Err(RuntimeSignal::Panic(PanicObj::new(
                            PanicType::WrongArgumentType,
                            format!(
                                "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                                other_type.get_type()
                            ),
                            state,
                        )));
                    }
                };

                let wrapper_res = if should_crate_raw {
                    FileWrapper::create(file_name_raw)
                } else {
                    FileWrapper::new(file_name_raw)
                };

                let wrapper = match wrapper_res {
                    Ok(wrapper) => wrapper,
                    Err(err_feedback) => {
                        return Ok(new_objectref(Object::new_error(
                            ErrorType::FileOpening,
                            err_feedback,
                            state.clone(),
                        )));
                    }
                };

                Ok(new_objectref(Object::Native(Box::new(NativeObject::File(
                    Box::new(wrapper),
                )))))
            }
            other_n_of_args => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "unexpected number of parameter for __open(). Expected: 1 or 2, got: '{}'",
                    other_n_of_args
                ),
                state,
            ))),
        }
    }
}
