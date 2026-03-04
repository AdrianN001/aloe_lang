use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    hashmap::{HashMap, HashPair},
};

impl Object {
    pub fn deep_copy(obj: ObjectRef) -> ObjectRef {
        let mut visited = std::collections::HashMap::new();
        Self::deep_copy_internal(obj, &mut visited)
    }

    fn deep_copy_internal(
        obj: ObjectRef,
        visited: &mut std::collections::HashMap<usize, ObjectRef>,
    ) -> ObjectRef {
        let ptr = Rc::as_ptr(&obj) as usize;

        // Zyklus erkannt
        if let Some(existing) = visited.get(&ptr) {
            return existing.clone();
        }

        // anti-cycle
        let obj_borrow = match obj.try_borrow() {
            Ok(ok) => ok,
            Err(_) => return Rc::new(RefCell::new(Object::NULL_OBJECT)),
        };

        let result = match &*obj_borrow {
            Object::Int(i) => Rc::new(RefCell::new(Object::Int(i.clone()))),

            Object::Bool(b) => Rc::new(RefCell::new(Object::Bool(b.clone()))),

            Object::String(s) => Rc::new(RefCell::new(Object::String(s.clone()))),

            Object::FloatObj(f) => Rc::new(RefCell::new(Object::FloatObj(f.clone()))),

            Object::Iterator(iter) => Rc::new(RefCell::new(Object::Iterator(iter.clone()))),

            Object::Null(_) => Rc::new(RefCell::new(Object::NULL_OBJECT)),

            Object::Array(arr) => {
                let new_array = Rc::new(RefCell::new(Object::Array(Array { items: vec![] })));

                visited.insert(ptr, new_array.clone());

                let items = arr
                    .items
                    .iter()
                    .map(|item| Self::deep_copy_internal(item.clone(), visited))
                    .collect();

                {
                    let mut arr = new_array.borrow_mut();
                    *arr = Object::Array(Array { items });
                }
                return new_array;
            }

            Object::HashMap(map) => {
                let new_map = Rc::new(RefCell::new(Object::HashMap(HashMap {
                    pairs: BTreeMap::new(),
                })));

                visited.insert(ptr, new_map.clone());

                let mut new_pairs = BTreeMap::new();

                for (key_hash, pair) in &map.pairs {
                    new_pairs.insert(
                        key_hash.clone(),
                        HashPair {
                            key: Self::deep_copy_internal(pair.key.clone(), visited),
                            value: Self::deep_copy_internal(pair.value.clone(), visited),
                        },
                    );
                }

                {
                    let mut map = new_map.borrow_mut();
                    *map = Object::HashMap(HashMap { pairs: new_pairs });
                }
                return new_map;
            }

            Object::Func(func) => Rc::new(RefCell::new(Object::Func(func.clone()))),

            Object::Err(e) => Rc::new(RefCell::new(Object::Err(e.clone()))),

            other => panic!("type {} cannot be copied", other.get_type()),
        };

        visited.insert(ptr, result.clone());

        result
    }
}
