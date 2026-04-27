use std::time::Instant;

use crate::object::future::task::Task;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FutureKind {
    Value(Task),
    Sleep(Instant),
}
