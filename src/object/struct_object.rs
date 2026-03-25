use std::collections::HashMap;

use crate::object::{
    Object, ObjectRef, error::panic_type::PanicType, new_objectref, panic_obj::PanicObj,
    stack_environment::EnvRef, state::StateRef, struct_model::MethodTableRef,
};

#[derive(PartialEq, Eq, Clone)]
pub struct StructObject {
    pub model: ObjectRef,
    pub model_name: String,

    pub attribute_table: HashMap<String, ObjectRef>,
    pub method_table: MethodTableRef,
}

impl StructObject {
    pub fn create_new_object(
        model: ObjectRef,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        let (attribute_list, method_table, name) = {
            let borrow = model.borrow();
            let model_raw = match &*borrow {
                Object::StructModel(struct_model) => struct_model,
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::IllegalExpression,
                        format!(
                            "expected to be the model a 'Struct Model', got: {}",
                            other_type.inspect()
                        ),
                        state,
                    ));
                }
            };

            (
                model_raw.attributes.clone(),
                model_raw.methods.clone(),
                model_raw.name.clone(),
            )
        };

        let attribute_table = {
            let mut attribute_map = HashMap::new();

            attribute_list.iter().for_each(|attribute_name| {
                attribute_map.insert(attribute_name.clone(), new_objectref(Object::NULL_OBJECT));
            });

            attribute_map
        };

        let mut new_object = Self {
            model: model.clone(),
            model_name: name,

            attribute_table,
            method_table,
        };

        new_object.init_object(args, &attribute_list, state)?;

        Ok(new_objectref(Object::StructObject(new_object)))
    }

    fn init_object(
        &mut self,
        args: &[ObjectRef],
        attribute_list: &[String],
        state: StateRef,
    ) -> Result<(), PanicObj> {
        let constructor_function = {
            let method_table_borrow = self.method_table.borrow();

            method_table_borrow
                .get("constructor")
                .map(|func| func.clone())
        };

        if let Some(_constr_func) = constructor_function {
            Ok(())
        } else if !args.is_empty() {
            self.default_constructor(args, attribute_list, state)
        } else {
            Ok(())
        }
    }

    fn default_constructor(
        &mut self,
        args: &[ObjectRef],
        attribute_list: &[String],
        state: StateRef,
    ) -> Result<(), PanicObj> {
        if args.len() != attribute_list.len() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for default constructor, got: {}.",
                    attribute_list.len(),
                    args.len()
                ),
                state,
            ));
        }

        attribute_list
            .iter()
            .zip(args)
            .for_each(|(attribute_name, argument)| {
                self.attribute_table
                    .insert(attribute_name.to_string(), argument.clone());
            });

        Ok(())
    }
}

impl StructObject {
    pub fn apply_attribute(
        &self,
        name: &str,
        _environ: EnvRef,
        _state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match self.attribute_table.get(name) {
            Some(attribute) => Ok(attribute.clone()),
            None => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("{} has no attribute: '{}'", self.model_name, name),
                _state,
            )),
        }
    }

    pub fn apply_method(
        name: &str,
        this: ObjectRef,
        args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        let method = {
            let this_borrow = this.borrow();

            let this_raw = match &*this_borrow {
                Object::StructObject(struct_obj) => struct_obj,
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "expected as the type of 'this': StructObject, got: '{}'",
                            other_type.inspect()
                        ),
                        state,
                    ));
                }
            };

            let method_table_borrow = this_raw.method_table.borrow();

            match method_table_borrow.get(name) {
                Some(requested_method) => requested_method.clone(),
                None => {
                    return Err(PanicObj::new(
                        PanicType::UnknownMethod,
                        format!("struct {} has no method '{}'().", this_raw.model_name, name),
                        state,
                    ));
                }
            }
        };

        let method_borrow = method.borrow();

        let func_obj = match &*method_borrow {
            Object::Func(func) => func,
            other_type => {
                return Err(PanicObj::new(
                    PanicType::IllegalExpression,
                    format!(
                        "expected function object for method, got: '{}'",
                        other_type.inspect()
                    ),
                    state,
                ));
            }
        };

        let args_with_this = Self::insert_this_reference_to_args(this, args);

        func_obj.apply(name.to_string(), &args_with_this, state)
    }

    fn insert_this_reference_to_args(this: ObjectRef, args: &[ObjectRef]) -> Vec<ObjectRef> {
        let mut new_args = vec![this];

        args.iter().for_each(|arg| {
            new_args.push(arg.clone());
        });

        new_args
    }
}

impl StructObject {
    pub fn inspect(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.model_name);
        buffer.push_str(" {");

        self.attribute_table
            .iter()
            .for_each(|(attribute_name, attribute)| {
                let attribute_str_format = {
                    let attribute_borrow = attribute.borrow();
                    attribute_borrow.inspect()
                };
                buffer.push_str(&format!(
                    " '{}' : {},",
                    &attribute_name, attribute_str_format
                ));
            });

        buffer.push_str(" }");

        buffer
    }

    pub fn get_type(&self) -> String {
        format!("<struct '{}'>", self.model_name)
    }
}
