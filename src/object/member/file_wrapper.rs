use std::io::{Read, Seek, SeekFrom, Write};

use crate::{
    object::{
        Object, ObjectRef,
        error::{error_type::ErrorType, panic_type::PanicType},
        future::{FutureObj, future_kind::FutureKind, future_state::FutureState},
        integer::Integer,
        native_object::{NativeObject, file::FileWrapper, path::PathWrapper},
        new_objectref,
        panic_obj::PanicObj,
        state::StateRef,
        string_obj::StringObj,
    },
    scheduler::{SCHEDULER_CHANNEL, TOKIO_RUNTIME, add_io_future, message_output::MessageOutput},
};

impl FileWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "get_path" => self.get_path(args, state),

            "read" => self.read(args, state),
            "read_async" => self.read_async(args, state),
            "write" => self.write(args, state),
            "write_byte" => self.write_byte(args, state),
            "seek" => self.seek(args, state),
            "close" => self.close(args, state),

            unknown_method => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method {} on {}", unknown_method, self.type_name()),
                state,
            )),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "is_open" => Ok(self.get_is_open()),
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

    pub fn get_path(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 0 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 0 parameters for file.get_path(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let wrapper = match PathWrapper::new(&self.path) {
            Ok(wrapper) => wrapper,
            Err(err_feedback) => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::PathResolve,
                    err_feedback,
                    state,
                )));
            }
        };

        Ok(new_objectref(Object::Native(Box::new(NativeObject::Path(
            wrapper,
        )))))
    }

    pub fn get_is_open(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self.get_is_open_raw()))
    }

    pub fn get_is_open_raw(&self) -> bool {
        self.native_file.is_some()
    }

    pub fn get_size(&self) -> ObjectRef {
        new_objectref(Object::Int(Integer {
            value: self.metadata.len() as i64,
        }))
    }

    // methods

    pub fn read(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 0 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!("expected 0 parameters for file.read(), got: {}", args.len()),
                state,
            ));
        }

        if self.is_write_only {
            return Ok(new_objectref(Object::new_error(
                ErrorType::FileMode,
                "file was opened as \"write-only\"".into(),
                state,
            )));
        }

        let mut native_file = match &self.native_file {
            Some(file) => file,
            None => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::FileIsClosed,
                    format!("{} is already closed.", self.inspect()),
                    state,
                )));
            }
        };

        let mut buffer = String::new();

        if let Err(error_feedback) = native_file.read_to_string(&mut buffer) {
            return Ok(new_objectref(Object::new_error(
                ErrorType::FileRead,
                error_feedback.to_string(),
                state,
            )));
        }

        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: buffer,
        }))))
    }

    pub fn write_byte(
        &mut self,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} parameters for file.write(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }

        let mut native_file = match &self.native_file {
            Some(file) => file,
            None => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::FileIsClosed,
                    format!("{} is already closed.", self.inspect()),
                    state,
                )));
            }
        };

        let arg_borrow = args[0].borrow();
        let bytes = match &*arg_borrow {
            Object::Buffer(buffer) => buffer.data.to_vec(),

            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected buffer as parameter for file.write_byte(), got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        match native_file.write(&bytes) {
            Ok(byte_written) => Ok(new_objectref(Object::Int(Integer {
                value: byte_written as i64,
            }))),

            Err(error_feedback) => Ok(new_objectref(Object::new_error(
                ErrorType::ErrorFromPanic,
                error_feedback.to_string(),
                state,
            ))),
        }
    }

    pub fn write(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} parameters for file.write(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();
        let content = match &*arg_borrow {
            Object::String(str) => &str.value,
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
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
                return Ok(new_objectref(Object::new_error(
                    ErrorType::FileIsClosed,
                    format!("{} is already closed.", self.inspect()),
                    state,
                )));
            }
        };

        match native_file.write(content.as_bytes()) {
            Ok(byte_written) => Ok(new_objectref(Object::Int(Integer {
                value: byte_written as i64,
            }))),

            Err(error_feedback) => Ok(new_objectref(Object::new_error(
                ErrorType::ErrorFromPanic,
                error_feedback.to_string(),
                state,
            ))),
        }
    }

    fn seek(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
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
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
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
                return Ok(new_objectref(Object::new_error(
                    ErrorType::FileIsClosed,
                    format!("{} is already closed.", self.inspect()),
                    state,
                )));
            }
        };

        if let Err(error_feedback) = native_file.seek(SeekFrom::Start(position as u64)) {
            return Ok(new_objectref(Object::new_error(
                ErrorType::FileSeek,
                error_feedback.to_string(),
                state,
            )));
        }

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    pub fn close(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 0 parameters for file.close(), got: {}",
                    args.len()
                ),
                state,
            ));
        }
        if !self.get_is_open_raw() {
            return Ok(new_objectref(Object::new_error(
                ErrorType::FileIsClosed,
                format!("{} is already closed.", self.inspect()),
                state,
            )));
        }

        let _ = self.native_file.take();

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    pub fn read_async(
        &mut self,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 0 parameters for file.read_async(), got: {}",
                    args.len()
                ),
                state,
            ));
        }
        if self.is_write_only {
            return Ok(new_objectref(Object::new_error(
                ErrorType::FileMode,
                "file was not opened with read flag".into(),
                state,
            )));
        }
        if !self.get_is_open_raw() {
            return Ok(new_objectref(Object::new_error(
                ErrorType::FileIsClosed,
                format!("{} is already closed.", self.inspect()),
                state,
            )));
        }

        let future = new_objectref(Object::Future(Box::new(FutureObj::new(
            FutureState::Pending(FutureKind::IO),
        ))));

        let future_id = {
            let future_borrow = future.borrow();
            if let Object::Future(future_obj) = &*future_borrow {
                future_obj.get_id()
            } else {
                panic!("Expected a Future object");
            }
        };

        add_io_future(future_id, future.clone());

        let path = self.path.clone();

        let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

        TOKIO_RUNTIME.with(|slot| {
            let runtime = slot.borrow();

            runtime.spawn(async move {
                let result = tokio::fs::read_to_string(path).await;
                match result {
                    Ok(content) => {
                        tx.send((future_id, MessageOutput::PlainText(content)))
                            .expect("Failed to send message");
                    }
                    Err(error) => {
                        tx.send((
                            future_id,
                            MessageOutput::Error((
                                ErrorType::FileRead,
                                error.to_string(),
                                "File.read_async()".to_string(),
                            )),
                        ))
                        .expect("Failed to send message");
                    }
                }
            });
        });

        Ok(future)
    }
}
