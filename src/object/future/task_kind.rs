use std::time::Instant;

use crate::object::future::task::TaskRef;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TaskKind {
    Sleep(Instant),
    Value(TaskRef),
}
