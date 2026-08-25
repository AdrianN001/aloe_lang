use crate::{
    artifact::artifact::ARTIFACT_MAGIC, artifact::artifact::Artifact, ast::program::Program,
    version::CURRENT_VERSION,
};

impl Program {
    pub fn to_artifact(&self) -> Artifact {
        Artifact {
            artifact_version: 1,
            aloe_version: CURRENT_VERSION,
            program: self.statements.clone(),
        }
    }
}

impl Artifact {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(ARTIFACT_MAGIC);
        bytes.extend_from_slice(&self.artifact_version.to_le_bytes());
        bytes.extend_from_slice(&self.aloe_version.to_bytes());
        let program_bytes = postcard::to_allocvec(&self.program)?;
        bytes.extend_from_slice(&program_bytes);
        Ok(bytes)
    }
}

pub fn write_artifact_to_file(
    artifact: &Artifact,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = artifact.to_bytes()?;
    std::fs::write(file_path, bytes)?;
    Ok(())
}
