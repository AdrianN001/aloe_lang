use std::{
    fs::{File, Metadata},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::object::{
    Object, ObjectRef,
    error::{error_type::ErrorType, panic_type::PanicType},
    integer::Integer,
    new_objectref,
    panic_obj::PanicObj,
    state::StateRef,
    string_obj::StringObj,
};

pub struct FileWrapper {
    native_file: File,
    metadata: Metadata,
    pub path: String,

    is_open: bool,

    pub raw_path: String,
    pub mode: String,
}

impl FileWrapper {
    pub fn new(path: String, mode: &str) -> Result<Self, String> {
        let file_with_error = match mode {
            "r" => File::options()
                .read(true)
                .write(false)
                .append(false)
                .create(false)
                .open(&path),
            "w" => File::options()
                .read(false)
                .write(true)
                .append(false)
                .create(false)
                .open(&path),
            "a" => File::options()
                .read(false)
                .write(false)
                .append(true)
                .create(false)
                .open(&path),
            "x" => File::create_new(&path),
            _ => return Err("illegal mode parameter".into()),
        };

        let file = match file_with_error {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };

        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(err) => return Err(err.to_string()),
        };

        let file_path = match Path::new(&path).to_str() {
            Some(file_path) => file_path.to_string(),
            None => return Err("could not get the path of file.".into()),
        };

        Ok(Self {
            native_file: file,
            metadata,
            path: file_path,

            is_open: true,

            raw_path: path,
            mode: mode.to_string(),
        })
    }

    pub fn type_name(&self) -> String {
        "<native object 'FileWrapper'>".into()
    }

    pub fn inspect(&self) -> String {
        format!("[FileWrapper for {:?}]", self.path)
    }

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
                    "unknown attribte {} on {}",
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
        new_objectref(Object::get_native_boolean_object(self.is_open))
    }

    pub fn get_is_open_raw(&self) -> bool {
        self.is_open
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
        let mut buffer = String::new();

        if let Err(error_feedback) = self.native_file.read_to_string(&mut buffer) {
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

        match self.native_file.write(content.as_bytes()) {
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

        if let Err(error_feedback) = self.native_file.seek(SeekFrom::Start(position as u64)) {
            return new_objectref(Object::new_error(
                ErrorType::FileSeek,
                error_feedback.to_string(),
                state,
            ));
        }

        new_objectref(Object::NULL_OBJECT)
    }
}

impl Clone for FileWrapper {
    fn clone(&self) -> Self {
        Self::new(self.raw_path.clone(), &self.mode.clone()).expect("clone error")
    }
}

impl PartialEq for FileWrapper {
    fn eq(&self, other: &FileWrapper) -> bool {
        self.path == other.path && self.mode == other.mode
    }
}

impl Eq for FileWrapper {}
