use std::collections::HashMap;

use crate::object::{
    Object, ObjectRef, new_objectref, panic_obj::PanicObj, stack_environment::EnvRef,
    state::StateRef, struct_model::MethodTableRef,
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

        if let Some(constr_func) = constructor_function {
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
    pub fn apply_attribute(&self, name: &str, environ: EnvRef, state: StateRef) -> ObjectRef {
        match self.attribute_table.get(name) {
            Some(attribute) => attribute.clone(),
            None => new_objectref(Object::NULL_OBJECT),
        }
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
