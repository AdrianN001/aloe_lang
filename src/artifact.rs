use std::path::PathBuf;

use crate::{artifact::build_flag::BuildFlag, ast::Parser, lexer::Lexer, module::Module};

pub mod artifact;
pub mod build_flag;
pub mod reader;
pub mod writer;

pub fn write_artifact_to_file(
    input_path: PathBuf,
    out_path: PathBuf,
    flag: BuildFlag,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_code = Module::read_source_file(&input_path)?;
    let mut parser = Parser::new(Lexer::new(source_code));

    if matches!(flag, BuildFlag::SizeOptimized) {
        parser.set_ignore_doc_comments(true);
        parser.set_strip_token_value(true);
    }

    let program = parser.into_a_program().unwrap();

    let artifact = program.to_artifact();
    let bytes = artifact.to_bytes()?;
    std::fs::write(out_path, bytes)?;
    Ok(())
}
