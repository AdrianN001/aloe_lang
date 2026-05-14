use crate::object::{ObjectRef, future::future_kind::FutureKind};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum FutureState {
    Pending(FutureKind),
    Ready(ObjectRef),

    #[default]
    Invalid,
}

impl FutureState {
    pub fn to_string(&self) -> String {
        match self {
            FutureState::Pending(kind) => format!("Pending({})", kind.to_string()),
            FutureState::Ready(_) => "Ready".to_string(),
            FutureState::Invalid => "Invalid".to_string(),
        }
    }
}
