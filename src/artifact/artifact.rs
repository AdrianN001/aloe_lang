use crate::{ast::statement::Statement, version::AloeVersion};

pub const ARTIFACT_MAGIC: &[u8; 4] = b"ALOE";

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Artifact {
    pub artifact_version: u16,
    pub aloe_version: AloeVersion,
    pub program: Vec<Statement>,
}
