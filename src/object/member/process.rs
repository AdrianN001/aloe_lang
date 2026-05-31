use crate::object::{
    Object, ObjectRef,
    array::Array,
    buffer::Buffer,
    error::{error_type::ErrorType, panic_type::PanicType},
    integer::Integer,
    native_object::{NativeObject, path::PathWrapper, process::CommandWrapper},
    new_objectref,
    panic_obj::PanicObj,
    state::StateRef,
    string_obj::StringObj,
};

impl CommandWrapper {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "program" => Ok(self.get_program()),
            "args" => Ok(self.get_args()),
            "current_dir" => Ok(self.get_current_dir()),
            "env" => Ok(self.get_envs()),

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

    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "add_arg" => self.add_arg(args, state),
            //"add_args" => {}
            "set_current_dir" => self.set_current_dir(args, state),
            "set_env" => self.set_env(args, state),
            //"spawn" => self.spawn(args, state),
            "output" => self.output(args, state),

            unknown_method => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method {} on {}", unknown_method, self.type_name()),
                state,
            )),
        }
    }
}

impl CommandWrapper {
    // attributes
    pub fn get_program(&self) -> ObjectRef {
        let program_name: String = self
            .native_cmd
            .get_program()
            .to_str()
            .expect("conversion error")
            .into();

        new_objectref(Object::String(Box::new(StringObj {
            value: program_name,
        })))
    }

    pub fn get_args(&self) -> ObjectRef {
        let args = self
            .native_cmd
            .get_args()
            .map(|arg| arg.to_str().expect("conversion error").to_string())
            .map(|arg_str| new_objectref(Object::String(Box::new(StringObj { value: arg_str }))))
            .collect();

        new_objectref(Object::Array(Box::new(Array { items: args })))
    }

    pub fn get_current_dir(&self) -> ObjectRef {
        let dir = match self.native_cmd.get_current_dir() {
            Some(dir) => dir,
            None => return new_objectref(Object::NULL_OBJECT),
        };

        let path_buffer = dir.to_path_buf();
        let display = path_buffer.to_str().expect("conversion error").into();

        let path_wrapper = PathWrapper {
            native_object: path_buffer,
            repr_str: display,
        };

        new_objectref(Object::Native(Box::new(NativeObject::Path(path_wrapper))))
    }

    pub fn get_envs(&self) -> ObjectRef {
        let envs = self
            .native_cmd
            .get_envs()
            .map(|(key, value)| {
                let key_str = key.to_str().expect("conversion error").to_string();
                let value_opt = match value {
                    Some(value) => Some(value.to_str().expect("conversion error").to_string()),
                    None => None,
                };

                (key_str, value_opt)
            })
            .map(|(key_str, value_opt)| {
                let key = new_objectref(Object::String(Box::new(StringObj { value: key_str })));
                let value = match value_opt {
                    Some(value_str) => {
                        new_objectref(Object::String(Box::new(StringObj { value: value_str })))
                    }
                    None => new_objectref(Object::NULL_OBJECT),
                };

                new_objectref(Object::Array(Box::new(Array {
                    items: vec![key, value],
                })))
            })
            .collect();

        new_objectref(Object::Array(Box::new(Array { items: envs })))
    }

    //methods

    pub fn add_arg(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 argument for Command.add_arg(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let arg = match &*args[0].borrow() {
            Object::String(str) => str.value.clone(),
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected str as first argument for Command.add_arg(), got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        self.native_cmd.arg(arg);

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    pub fn set_current_dir(
        &mut self,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 argument for Command.set_current_dir(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let path = match &*args[0].borrow() {
            Object::String(str) => str.value.clone(),
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected str as first argument for Command.set_current_dir(), got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        self.native_cmd.current_dir(path);

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    pub fn set_env(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 2 argument for Command.set_env(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let key = match &*args[0].borrow() {
            Object::String(str) => str.value.clone(),
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected str as first argument for Command.set_env(), got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        let value = match &*args[1].borrow() {
            Object::String(str) => str.value.clone(),
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected str as second argument for Command.set_env(), got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        self.native_cmd.env(key, value);

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    pub fn output(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 0 argument for Command.spawn(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let output = match self.native_cmd.output() {
            Ok(output) => output,
            Err(error_feedback) => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::Command,
                    error_feedback.to_string(),
                    state,
                )));
            }
        };

        let status_code = match output.status.code() {
            Some(value) => new_objectref(Object::Int(Integer {
                value: value as i64,
            })),
            None => new_objectref(Object::NULL_OBJECT),
        };

        let std_out = new_objectref(Object::Buffer(Box::new({
            let out = output.stdout.into_boxed_slice();
            let size = out.len();

            Buffer { data: out, size }
        })));

        let std_err = new_objectref(Object::Buffer(Box::new({
            let out = output.stderr.into_boxed_slice();
            let size = out.len();

            Buffer { data: out, size }
        })));

        Ok(new_objectref(Object::Array(Box::new(Array {
            items: vec![status_code, std_out, std_err],
        }))))
    }

    pub fn to_bool(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(false))
    }

    pub fn to_bool_raw(&self) -> bool {
        false
    }
}
