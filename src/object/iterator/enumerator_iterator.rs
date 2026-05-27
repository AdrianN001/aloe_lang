use crate::object::{Object, ObjectRef, array::Array, integer::Integer, new_objectref};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EnumeratorIterator {
    pub list: Vec<ObjectRef>,
    pub index: usize,
}

impl EnumeratorIterator {
    pub fn _has_next_raw(&self) -> bool {
        self.index < self.list.len()
    }

    pub fn _has_next(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self._has_next_raw()))
    }
    pub fn _next(&mut self) -> Option<ObjectRef> {
        let current_index = self.index;

        if current_index >= self.list.len() {
            return None;
        }

        self.index += 1;

        let new_arr = Object::Array(Box::new(Array {
            items: vec![
                new_objectref(Object::Int(Integer {
                    value: current_index as i64,
                })),
                self.list[current_index].clone(),
            ],
        }));

        Some(new_objectref(new_arr))
    }
}
