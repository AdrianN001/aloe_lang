use crate::object::{ObjectRef, future::future_kind::FutureKind};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FutureState {
    Pending(FutureKind),
    Ready(ObjectRef),
}
