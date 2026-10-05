use std::path::PathBuf;

use crate::{
    artifact::build_flag::BuildFlag, ast::Parser, lexer::Lexer, module::Module,
    object::panic_obj::RuntimeSignal,
};

pub mod artifact;
pub mod build_flag;
pub mod reader;
pub mod writer;

pub fn write_artifact_to_file(
    input_path: PathBuf,
    out_path: PathBuf,
    flag: BuildFlag,
) -> Result<(), RuntimeSignal> {
    let source_code = match Module::read_source_file(&input_path) {
        Ok(source_code) => source_code,
        Err(err) => return Err(RuntimeSignal::GenericError(err)),
    };
    let mut parser = Parser::new(Lexer::new(source_code));

    if matches!(flag, BuildFlag::SizeOptimized) {
        parser.set_ignore_doc_comments(true);
        parser.set_strip_token_value(true);
    }

    let program = match parser.into_a_program() {
        Ok(program) => program,
        Err(err) => return Err(err.into()),
    };

    let artifact = program.to_artifact();
    let bytes = match artifact.to_bytes() {
        Ok(bytes) => bytes,
        Err(err) => return Err(RuntimeSignal::GenericError(err)),
    };
    std::fs::write(out_path, bytes);
    Ok(())
}
