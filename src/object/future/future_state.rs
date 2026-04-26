use crate::object::{ObjectRef, future::task::Task};



#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FutureState{
    Pending(Task),
    Ready(ObjectRef),
}
