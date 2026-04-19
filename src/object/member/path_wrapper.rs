use std::fs;

use crate::object::{
    Object, ObjectRef,
    array::Array,
    error::panic_type::PanicType,
    native_object::{NativeObject, path::PathWrapper},
    new_objectref,
    panic_obj::PanicObj,
    state::StateRef,
    string_obj::StringObj,
};

impl PathWrapper {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        println!("called: {}", name);
        match name {
            "is_absolute" => Ok(self.is_absolute()),
            "is_relative" => Ok(self.is_relative()),
            "exists" => Ok(self.exists()),
            "is_file" => Ok(self.is_file()),
            "is_dir" => Ok(self.is_dir()),

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
        &self,
        name: &str,
        _args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "as_absolute" => self.as_absolute(state),
            "as_str" => self.as_str(),
            "parent" => self.parent(state),
            "children" => self.children(state),

            unknown_method => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method {} on {}", unknown_method, self.type_name()),
                state,
            )),
        }
    }
}

impl PathWrapper {
    // attributes

    pub fn is_absolute(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(
            self.native_object.is_absolute(),
        ))
    }

    pub fn is_relative(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(
            self.native_object.is_relative(),
        ))
    }

    pub fn exists(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(
            self.native_object.exists(),
        ))
    }

    pub fn exists_raw(&self) -> bool {
        self.native_object.exists()
    }

    pub fn is_file(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(
            self.native_object.is_file(),
        ))
    }

    pub fn is_dir(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(
            self.native_object.is_dir(),
        ))
    }

    // methods

    pub fn as_absolute(&self, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let abs_path = match std::fs::canonicalize(&self.native_object) {
            Ok(ok_value) => ok_value,
            Err(err_feedback) => {
                return Err(PanicObj::new(
                    PanicType::PathResolve,
                    err_feedback.to_string(),
                    state,
                ));
            }
        };

        let repr_str = match &abs_path.to_str() {
            Some(some_val) => some_val.to_string(),
            None => {
                return Err(PanicObj::new(
                    PanicType::PathResolve,
                    format!("could not get the absolute path of ['{}'].", self.repr_str),
                    state,
                ));
            }
        };

        let wrapper = PathWrapper {
            native_object: abs_path,
            repr_str,
        };

        Ok(new_objectref(Object::Native(NativeObject::Path(wrapper))))
    }

    pub fn parent(&self, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match self.native_object.parent() {
            Some(existing_parent) => {
                let repr_str = match existing_parent.to_str() {
                    Some(some_val) => some_val,
                    None => {
                        return Err(PanicObj::new(
                            PanicType::PathResolve,
                            format!("could not resolve the parent of ['{}'].", self.repr_str),
                            state,
                        ));
                    }
                };

                let wrapper = match PathWrapper::new(repr_str) {
                    Ok(wrapper) => wrapper,
                    Err(err_feedback) => {
                        return Err(PanicObj::new(PanicType::PathResolve, err_feedback, state));
                    }
                };
                Ok(new_objectref(Object::Native(NativeObject::Path(wrapper))))
            }
            None => {
                let wrapper = match PathWrapper::new("") {
                    Ok(wrapper) => wrapper,
                    Err(err_feedback) => {
                        return Err(PanicObj::new(PanicType::PathResolve, err_feedback, state));
                    }
                };

                Ok(new_objectref(Object::Native(NativeObject::Path(wrapper))))
            }
        }
    }

    pub fn children(&self, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let children = match fs::read_dir(&self.native_object) {
            Ok(ok_value) => ok_value,
            Err(err_feedback) => {
                return Err(PanicObj::new(
                    PanicType::PathChildResolve,
                    err_feedback.to_string(),
                    state,
                ));
            }
        };

        let mut children_objects = vec![];

        for child in children {
            let ok_child = match child {
                Ok(ok_value) => ok_value,
                Err(err_feedback) => {
                    return Err(PanicObj::new(
                        PanicType::PathChildResolve,
                        err_feedback.to_string(),
                        state,
                    ));
                }
            };

            let path = ok_child.path();
            let repr_str = ok_child.path().display().to_string();

            children_objects.push(new_objectref(Object::Native(NativeObject::Path(
                PathWrapper {
                    native_object: path,
                    repr_str,
                },
            ))));
        }

        Ok(new_objectref(Object::Array(Array {
            items: children_objects,
        })))
    }

    pub fn as_str(&self) -> Result<ObjectRef, PanicObj> {
        Ok(new_objectref(Object::String(StringObj {
            value: self.repr_str.clone(),
        })))
    }
}
