use std::{cell::RefCell, rc::Rc};

use crate::object::error::panic_type::PanicType;
use crate::object::hashmap::HashPair;
use crate::object::panic_obj::PanicObj;
use crate::object::stack_environment::EnvRef;
use crate::object::state::StateRef;
use crate::object::string_obj::StringObj;
use crate::{
    ast::expression::index_expression::IndexExpression,
    object::{Object, ObjectRef, hashable::Hashable},
};

impl IndexExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let left_expr = self.left.evaluate(environ.clone(), state.clone())?;
        let index = self.right.evaluate(environ.clone(), state.clone())?;

        match (&*left_expr.borrow(), &*index.borrow()) {
            (_, Object::ReturnVal(ret_val)) => Ok(ret_val.unwrap_to_value().clone()),
            (Object::Array(arr), Object::Int(index)) => {
                let arr_interior_value = &arr.items;
                let mut index_interior_value = index.value;

                if index_interior_value < 0 {
                    index_interior_value += arr.items.len() as i64;
                }

                if index_interior_value > (arr_interior_value.len() as i64) - 1
                    || index_interior_value < 0
                {
                    return Err(PanicObj::new(
                        PanicType::IndexOutOfBound,
                        format!(
                            "indexing an array of size '{}' with index '{}' is illegal.",
                            arr.items.len(),
                            index_interior_value
                        ),
                        state,
                    ));
                }

                Ok(arr_interior_value[index_interior_value as usize].clone())
            }
            (Object::String(str), Object::Int(index)) => {
                let arr_interior_value = &str.value;
                let mut index_interior_value = index.value;

                if index_interior_value < 0 {
                    index_interior_value += str.value.len() as i64;
                }

                if index_interior_value > (arr_interior_value.len() as i64) - 1
                    || index_interior_value < 0
                {
                    return Err(PanicObj::new(
                        PanicType::IndexOutOfBound,
                        format!(
                            "indexing a string of size '{}' with index '{}' is illegal.",
                            str.value.len(),
                            index_interior_value
                        ),
                        state,
                    ));
                }

                Ok(Rc::new(RefCell::new(Object::String(StringObj {
                    value: arr_interior_value
                        .chars()
                        .nth(index_interior_value as usize)
                        .unwrap()
                        .to_string(),
                }))))
            }
            (Object::HashMap(map), _) => Ok(map.get([index.clone()].as_ref(), state)),
            _ => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "index operator not supported on: {}",
                    index.borrow().get_type()
                ),
                state.clone(),
            )),
        }
    }

    pub fn evaluate_value_assign(
        &self,
        environ: EnvRef,
        rvalue: ObjectRef,
        state: StateRef,
    ) -> Result<(), PanicObj> {
        let left_expr = self.left.evaluate(environ.clone(), state.clone())?;
        let index = self.right.evaluate(environ.clone(), state.clone())?;

        let mut left_borrow = left_expr.borrow_mut();
        let index_borrow = index.borrow();

        match &mut *left_borrow {
            Object::Array(arr) => {
                let idx = match &*index_borrow {
                    Object::Int(i) => i.value,
                    _ => {
                        return Err(PanicObj::new_simple(
                            PanicType::WrongArgumentType,
                            "Index must be integer",
                            state.clone(),
                        ));
                    }
                };

                let len = arr.items.len() as i64;

                let real_index = if idx < 0 { len + idx } else { idx };

                if real_index < 0 || real_index >= len {
                    return Err(PanicObj::new_simple(
                        PanicType::IndexOutOfBound,
                        "out of bounds panic",
                        state.clone(),
                    ));
                }

                arr.items[real_index as usize] = rvalue;
                Ok(())
            }

            Object::HashMap(map) => {
                let hashed_object = match index_borrow.hash() {
                    Ok(ok_value) => ok_value,
                    Err(err_feedback) => {
                        return Err(PanicObj::new(
                            PanicType::ObjectNotHashable,
                            err_feedback,
                            state.clone(),
                        ));
                    }
                };

                map.pairs.insert(
                    hashed_object,
                    HashPair {
                        key: index.clone(),
                        value: rvalue.clone(),
                    },
                );

                Ok(())
            }

            _ => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!("index operator not supported on {}", left_borrow.get_type()),
                state.clone(),
            )),
        }
    }
}
