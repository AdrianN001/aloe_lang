use crate::object::{
    ObjectRef,
    iterator::{
        enumerator_iterator::EnumeratorIterator, list_based_iterator::ListBasedIterator,
        range_based_iterator::RangeBasedIterator, zip_iterator::ZipIterator,
    },
};

pub mod enumerator_iterator;
pub mod list_based_iterator;
pub mod range_based_iterator;
pub mod zip_iterator;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Iterator {
    RangeBasedIterator(RangeBasedIterator),
    ListBasedIterator(ListBasedIterator),
    EnumeratorIterator(EnumeratorIterator),
    ZipIterator(ZipIterator),
}

impl Iterator {
    pub fn get_type(&self) -> String {
        "iterator object".into()
    }

    pub fn inspect(&self) -> String {
        self.get_type()
    }

    pub fn _has_next_raw(&self) -> bool {
        match self {
            Iterator::RangeBasedIterator(range_based_iterator) => {
                range_based_iterator._has_next_raw()
            }
            Iterator::ListBasedIterator(list_based_iterator) => list_based_iterator._has_next_raw(),
            Iterator::EnumeratorIterator(enumerator_iterator) => {
                enumerator_iterator._has_next_raw()
            }
            Iterator::ZipIterator(zip_iterator) => zip_iterator._has_next_raw(),
        }
    }

    pub fn _has_next(&self) -> ObjectRef {
        match self {
            Iterator::RangeBasedIterator(range_based_iterator) => range_based_iterator._has_next(),
            Iterator::ListBasedIterator(list_based_iterator) => list_based_iterator._has_next(),
            Iterator::EnumeratorIterator(enumerator_iterator) => enumerator_iterator._has_next(),
            Iterator::ZipIterator(zip_iterator) => zip_iterator._has_next(),
        }
    }

    pub fn _next(&mut self) -> Option<ObjectRef> {
        match self {
            Iterator::RangeBasedIterator(range_based_iterator) => range_based_iterator._next(),
            Iterator::ListBasedIterator(list_based_iterator) => list_based_iterator._next(),
            Iterator::EnumeratorIterator(enumerator_iterator) => enumerator_iterator._next(),
            Iterator::ZipIterator(zip_iterator) => zip_iterator._next(),
        }
    }
}
