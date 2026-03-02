use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    float_obj::FloatObj,
    hashmap::{HashMap, HashPair},
};

impl Object {
    pub fn deep_copy(obj: &ObjectRef) -> ObjectRef {
        match &*obj.borrow() {
            Object::Int(i) => Rc::new(RefCell::new(Object::Int(i.clone()))),

            Object::Bool(b) => Rc::new(RefCell::new(Object::Bool(b.clone()))),

            Object::String(s) => Rc::new(RefCell::new(Object::String(s.clone()))),

            Object::FloatObj(float) => Rc::new(RefCell::new(Object::FloatObj(float.clone()))),

            Object::Iterator(iter) => Rc::new(RefCell::new(Object::Iterator(iter.clone()))),

            Object::Null(_) => Rc::new(RefCell::new(Object::NULL_OBJECT)),

            Object::Array(arr) => {
                let new_items = arr
                    .items
                    .iter()
                    .map(|item| Object::deep_copy(item))
                    .collect();

                Rc::new(RefCell::new(Object::Array(Array { items: new_items })))
            }

            Object::HashMap(map) => {
                let mut new_pairs = BTreeMap::new();

                for (key_hash, pair) in &map.pairs {
                    new_pairs.insert(
                        key_hash.clone(),
                        HashPair {
                            key: Object::deep_copy(&pair.key),
                            value: Object::deep_copy(&pair.value),
                        },
                    );
                }

                Rc::new(RefCell::new(Object::HashMap(HashMap { pairs: new_pairs })))
            }

            Object::Func(func) => {
                // Achtung: Environment nicht deep copy!
                Rc::new(RefCell::new(Object::Func(func.clone())))
            }

            Object::Err(e) => Rc::new(RefCell::new(Object::Err(e.clone()))),

            other_type => panic!("type {} can not be copied", other_type.get_type()),
        }
    }
}
