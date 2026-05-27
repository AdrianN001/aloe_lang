use crate::object::{Object, ObjectRef, array::Array, new_objectref};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ZipIterator {
    pub list: Vec<ObjectRef>,
    pub index: usize,
    pub min_length: usize,
}

impl ZipIterator {
    pub fn _has_next_raw(&self) -> bool {
        self.index < self.min_length
    }

    pub fn _has_next(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self._has_next_raw()))
    }
    pub fn _next(&mut self) -> Option<ObjectRef> {
        let current_index = self.index;

        if current_index >= self.min_length {
            return None;
        }

        self.index += 1;

        let raw_list = self
            .list
            .iter()
            .map(|array_value| {
                let arr_value_borrow = array_value.borrow();

                let items = match &*arr_value_borrow {
                    Object::Array(arr) => &arr.items,
                    _ => unreachable!(),
                };

                let current_item = items[current_index].clone();

                current_item
            })
            .collect::<Vec<ObjectRef>>();

        let arr = Object::Array(Box::new(Array { items: raw_list }));

        Some(new_objectref(arr))
    }
}
