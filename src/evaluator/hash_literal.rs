use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{
    ast::expression::hash_map_literal::HashMapLiteral,
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        hashmap::{HashMap, HashPair},
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl HashMapLiteral {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let mut pairs = BTreeMap::new();

        for (k, v) in &self.pairs {
            let key = k.evaluate(environ.clone(), state.clone())?;

            if let Object::ReturnVal(_) = &*key.borrow() {
                return Ok(key.clone());
            }

            let value = v.evaluate(environ.clone(), state.clone())?;

            if let Object::ReturnVal(_) = &*value.borrow() {
                return Ok(value.clone());
            }

            let hashed_key = match key.borrow().hash() {
                Ok(ok_value) => ok_value,
                Err(err_feedback) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::ObjectNotHashable,
                        err_feedback,
                        state.clone(),
                    )));
                }
            };

            pairs.insert(
                hashed_key,
                HashPair {
                    key: key.clone(),
                    value: value.clone(),
                },
            );
        }

        Ok(Rc::new(RefCell::new(Object::HashMap(HashMap { pairs }))))
    }
}
