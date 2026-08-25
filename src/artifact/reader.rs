use crate::{
    artifact::artifact::{ARTIFACT_MAGIC, Artifact},
    ast::{program::Program, statement::Statement},
    version::AloeVersion,
};

impl Program {
    pub fn from_artifact(artifact: Artifact) -> Result<Self, Box<dyn std::error::Error>> {
        Program::check_artifact(&artifact)?;

        Ok(Program {
            statements: artifact.program,
        })
    }

    fn check_artifact(_artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>> {
        //TODO: Implement artifact version and aloe version checks here
        Ok(())
    }
}

impl Artifact {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if bytes.len() < 7 {
            return Err("Invalid artifact: too short".into());
        }

        if &bytes[0..4] != ARTIFACT_MAGIC {
            return Err("Invalid artifact: incorrect magic number".into());
        }

        let artifact_version = u16::from_le_bytes([bytes[4], bytes[5]]);
        let aloe_version = AloeVersion::from_bytes(&bytes[6..7]).unwrap_or(AloeVersion::Avocado);

        let program_bytes = &bytes[7..];
        let program: Vec<Statement> = postcard::from_bytes(program_bytes)?;

        Ok(Artifact {
            artifact_version,
            aloe_version,
            program,
        })
    }
}

pub fn read_artifact_from_file(file_path: &str) -> Result<Artifact, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(file_path)?;
    Artifact::from_bytes(&bytes)
}
