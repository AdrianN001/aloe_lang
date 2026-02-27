use crate::object::{ObjectRef, iterator::{list_based_iterator::ListBasedIterator, range_based_iterator::RangeBasedIterator}};

pub mod list_based_iterator;
pub mod range_based_iterator;

#[derive(PartialEq, Eq, Clone)]
pub enum Iterator {
    RangeBasedIterator(RangeBasedIterator),
    ListBasedIterator(ListBasedIterator)
}

impl Iterator {
    pub fn get_type(&self) -> String {
        "iterator object".into()
    }

    pub fn inspect(&self) -> String {
        self.get_type()
    }

    pub fn _has_next(&self) -> ObjectRef {
        match self {
            Iterator::RangeBasedIterator(range_based_iterator) => range_based_iterator._has_next(),
            Iterator::ListBasedIterator(list_based_iterator) => list_based_iterator._has_next()
        }
    }

    pub fn _next(&mut self) -> Option<ObjectRef> {
        match self {
            Iterator::RangeBasedIterator(range_based_iterator) => range_based_iterator._next(),
            Iterator::ListBasedIterator(list_based_iterator) => list_based_iterator._next(),
        }
    }
}
