use std::io::{Read, Seek, SeekFrom, Write};

use crate::object::{
    Object, ObjectRef,
    error::{error_type::ErrorType, panic_type::PanicType},
    integer::Integer,
    native_object::file::FileWrapper,
    new_objectref,
    panic_obj::PanicObj,
    state::StateRef,
    string_obj::StringObj,
};

impl FileWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "read" => Ok(self.read(state)),
            "write" => Ok(self.write(args, state)),
            "seek" => Ok(self.seek(args, state)),
            "close" => Ok(self.close(state)),

            unknown_method => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method {} on {}", unknown_method, self.type_name()),
                state,
            )),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "path" => Ok(self.get_path()),
            "is_open" => Ok(self.get_is_open()),
            "mode" => Ok(self.get_mode()),
            "size" => Ok(self.get_size()),

            unknown_attribute => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!(
                    "unknown attribute {} on {}",
                    unknown_attribute,
                    self.type_name()
                ),
                state,
            )),
        }
    }
}

impl FileWrapper {
    // attributes

    pub fn get_path(&self) -> ObjectRef {
        new_objectref(Object::String(StringObj {
            value: self.path.clone(),
        }))
    }

    pub fn get_is_open(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self.get_is_open_raw()))
    }

    pub fn get_is_open_raw(&self) -> bool {
        self.native_file.is_some()
    }

    pub fn get_mode(&self) -> ObjectRef {
        new_objectref(Object::String(StringObj {
            value: self.mode.clone(),
        }))
    }

    pub fn get_size(&self) -> ObjectRef {
        new_objectref(Object::Int(Integer {
            value: self.metadata.len() as i64,
        }))
    }

    // methods

    pub fn read(&mut self, state: StateRef) -> ObjectRef {
        if self.mode != "r" {
            return new_objectref(Object::new_error(
                ErrorType::FileMode,
                "file was not opened with read flag".into(),
                state,
            ));
        }

        let mut native_file = match &self.native_file {
            Some(file) => file,
            None => {
                return new_objectref(Object::new_error(
                    ErrorType::FileIsClosed,
                    format!("{} is already closed.", self.inspect()),
                    state,
                ));
            }
        };

        let mut buffer = String::new();

        if let Err(error_feedback) = native_file.read_to_string(&mut buffer) {
            return new_objectref(Object::new_error(
                ErrorType::FileRead,
                error_feedback.to_string(),
                state,
            ));
        }

        new_objectref(Object::String(StringObj { value: buffer }))
    }

    pub fn write(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return new_objectref(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected {} parameters for file.write(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }
        if self.mode != "w" && self.mode != "a" {
            return new_objectref(Object::new_error(
                ErrorType::FileMode,
                "file was not opened with write/append flag".into(),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();
        let content = match &*arg_borrow {
            Object::String(str) => &str.value,
            other_type => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected str as parameter for file.write(), got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        let mut native_file = match &self.native_file {
            Some(file) => file,
            None => {
                return new_objectref(Object::new_error(
                    ErrorType::FileIsClosed,
                    format!("{} is already closed.", self.inspect()),
                    state,
                ));
            }
        };

        match native_file.write(content.as_bytes()) {
            Ok(byte_written) => new_objectref(Object::Int(Integer {
                value: byte_written as i64,
            })),

            Err(error_feedback) => new_objectref(Object::new_error(
                ErrorType::ErrorFromPanic,
                error_feedback.to_string(),
                state,
            )),
        }
    }

    fn seek(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return new_objectref(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected {} parameters for file.write(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();
        let position = match &*arg_borrow {
            Object::Int(int) => int.value,
            other_type => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected int as parameter for file.seek(), got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        let mut native_file = match &self.native_file {
            Some(file) => file,
            None => {
                return new_objectref(Object::new_error(
                    ErrorType::FileIsClosed,
                    format!("{} is already closed.", self.inspect()),
                    state,
                ));
            }
        };

        if let Err(error_feedback) = native_file.seek(SeekFrom::Start(position as u64)) {
            return new_objectref(Object::new_error(
                ErrorType::FileSeek,
                error_feedback.to_string(),
                state,
            ));
        }

        new_objectref(Object::NULL_OBJECT)
    }

    pub fn close(&mut self, state: StateRef) -> ObjectRef {
        if !self.get_is_open_raw() {
            return new_objectref(Object::new_error(
                ErrorType::FileIsClosed,
                format!("{} is already closed.", self.inspect()),
                state,
            ));
        }

        let _ = self.native_file.take();

        new_objectref(Object::NULL_OBJECT)
    }
}
