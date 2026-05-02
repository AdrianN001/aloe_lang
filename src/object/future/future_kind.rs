use std::time::Instant;

use crate::object::future::task::TaskRef;

#[derive(Debug)]
pub enum FutureKind {
    Value(TaskRef),
    Sleep(Instant),
    FileIO,
}

impl Clone for FutureKind {
    fn clone(&self) -> Self {
        match self {
            FutureKind::Value(task_ref) => FutureKind::Value(task_ref.clone()),
            FutureKind::Sleep(instant) => FutureKind::Sleep(*instant),
            FutureKind::FileIO => FutureKind::FileIO,
        }
    }
}

impl PartialEq for FutureKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FutureKind::Value(task_ref1), FutureKind::Value(task_ref2)) => task_ref1 == task_ref2,
            (FutureKind::Sleep(instant1), FutureKind::Sleep(instant2)) => instant1 == instant2,
            (FutureKind::FileIO, FutureKind::FileIO) => panic!("Cannot compare FileIO future kind"),
            _ => false,
        }
    }
}

impl Eq for FutureKind {}
