use std::time::Instant;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TaskKind {
    Sleep(Instant),
}
