use crate::object::{ObjectRef, future::future_kind::FutureKind};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum FutureState {
    Pending(FutureKind),
    Ready(ObjectRef),

    #[default]
    Invalid,
}
