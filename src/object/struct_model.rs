use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

pub type MethodTableRef = Rc<RefCell<HashMap<String, ObjectRef>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct StructModel {
    pub name: String,

    pub attributes: Vec<String>,
    pub methods: MethodTableRef,
}

impl StructModel {
    pub fn get_type(&self) -> String {
        "<type struct>".to_string()
    }

    pub fn inspect(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("struct ");
        buffer.push_str(&self.name);
        buffer.push('{');

        if !self.attributes.is_empty() {
            buffer.push('\n');
        }
        self.attributes.iter().for_each(|attribute| {
            buffer.push('\t');
            buffer.push_str(attribute);
            buffer.push(',');
            buffer.push('\n');
        });
        buffer.push('}');
        buffer.push(';');

        buffer
    }

    pub fn call_method(
        &self,
        method_name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let method_searched = {
            let method_table_borrow = self.methods.borrow();

            match method_table_borrow.get(method_name) {
                Some(method) => method.clone(),
                None => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::UnknownMethod,
                        format!("struct {} has no method '{}'", self.name, method_name),
                        state,
                    )));
                }
            }
        };

        match &*method_searched.borrow() {
            Object::Func(function) => function.apply(method_name.to_string(), &args, state.clone()),
            Object::AsyncFunc(async_function) => {
                async_function.apply(method_name.to_string(), &args, state.clone())
            }
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::NonfunctionalObjectCalled,
                    format!(
                        "{}::{} is not callable. it is type of '{}'",
                        self.name,
                        method_name,
                        other_type.get_type()
                    ),
                    state,
                )));
            }
        }
    }
}
