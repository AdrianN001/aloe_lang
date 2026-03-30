use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    native_object::{NativeObject, file::FileWrapper},
    new_objectref,
    panic_obj::PanicObj,
    state::StateRef,
};

pub fn open_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
    match args.len() {
        1 => {
            // open("file_name");
            let file_name_borrow = args[0].borrow();

            let file_name_raw = match &*file_name_borrow {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            };

            let mode = "r";

            let wrapper = match FileWrapper::new(file_name_raw, mode) {
                Ok(wrapper) => wrapper,
                Err(err_feedback) => {
                    return Err(PanicObj::new(PanicType::FileOpen, err_feedback, state));
                }
            };

            Ok(new_objectref(Object::Native(NativeObject::File(wrapper))))
        }

        2 => {
            // open("file_name", "mode");
            let file_name_borrow = args[0].borrow();
            let mode_borrow = args[1].borrow();

            let file_name_raw = match &*file_name_borrow {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            };

            let mode_raw = match &*mode_borrow {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            };

            let wrapper = match FileWrapper::new(file_name_raw, &mode_raw) {
                Ok(wrapper) => wrapper,
                Err(err_feedback) => {
                    return Err(PanicObj::new(PanicType::FileOpen, err_feedback, state));
                }
            };

            Ok(new_objectref(Object::Native(NativeObject::File(wrapper))))
        }
        other_n_of_args => Err(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "unexpected number of parameter for __open(). Expected: 'str', got: '{}'",
                other_n_of_args
            ),
            state,
        )),
    }
}
