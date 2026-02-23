use crate::object::hashmap::HashKey;

pub trait Hashable {
    fn hash(&self) -> HashKey;
}
