use std::{cell::RefCell, rc::Rc};

use crate::object::stack_environment::EnvRef;
use crate::object::string_obj::StringObj;
use crate::{
    ast::expression::index_expression::IndexExpression,
    object::{Object, ObjectRef, hashable::Hashable},
};

impl IndexExpression {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let left_expr = self.left.evaluate(environ.clone())?;
        let index = self.right.evaluate(environ.clone())?;

        match (&*left_expr.borrow(), &*index.borrow()) {
            (Object::Array(arr), Object::Int(index)) => {
                let arr_interior_value = &arr.items;
                let mut index_interior_value = index.value;

                if index_interior_value < 0 {
                    index_interior_value = arr.items.len() as i64 - index_interior_value;
                }

                if index_interior_value > (arr_interior_value.len() as i64) - 1 {
                    return Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)));
                }

                Ok(arr_interior_value[index_interior_value as usize].clone())
            }
            (Object::String(str), Object::Int(index)) => {
                let arr_interior_value = &str.value;
                let mut index_interior_value = index.value;

                if index_interior_value < 0 {
                    index_interior_value = str.value.len() as i64 - index_interior_value;
                }

                if index_interior_value > (arr_interior_value.len() as i64) - 1 {
                    return Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)));
                }

                Ok(Rc::new(RefCell::new(Object::String(StringObj {
                    value: arr_interior_value
                        .chars()
                        .nth(index_interior_value as usize)
                        .unwrap()
                        .to_string(),
                }))))
            }
            (Object::HashMap(map), key_object) => {
                if !key_object.is_hashable() {
                    return Err(format!(
                        "key object is not hashable: {}",
                        index.borrow().get_type()
                    ));
                }

                let hashed_object = match key_object {
                    Object::String(str) => str.hash(),
                    Object::Int(int) => int.hash(),
                    Object::Bool(bool) => bool.hash(),
                    _ => panic!(),
                };

                if !map.pairs.contains_key(&hashed_object) {
                    return Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)));
                }

                Ok(map.pairs[&hashed_object].value.clone())
            }
            _ => Err(format!(
                "index operator not supported: {}",
                index.borrow().get_type()
            )),
        }
    }
}
