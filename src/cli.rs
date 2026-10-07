use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{
    artifact::{build_flag::BuildFlag, write_artifact_to_file},
    doc::{html_document_a_project, html_document_a_single_file, json_document_a_single_file},
    object::panic_obj::RuntimeSignal,
    repl::start_repl,
    script::{run_artifact, run_script},
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        file: Option<PathBuf>,

        #[arg(short, long)]
        artifact: bool,
    },

    Doc {
        file: PathBuf,

        #[arg(long)]
        project: bool,

        #[arg(long)]
        html: bool,

        #[arg(short, long)]
        json: bool,

        output_dir: Option<PathBuf>,
    },

    Build {
        file: PathBuf,
        out: PathBuf,
    },

    Repl,
}

pub fn parse_cli() -> Result<(), RuntimeSignal> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run {
            file: file_opt,
            artifact,
        } => match file_opt {
            Some(file) => {
                let _result = {
                    if artifact {
                        run_artifact(&file)
                    } else {
                        run_script(&file)
                    }
                };

                Ok(())
            }
            None => {
                //todo run main.aloe
                Ok(())
            }
        },
        Command::Doc {
            file,
            project: _,
            html,
            json,
            output_dir,
        } => {
            if let Some(output_dir) = output_dir {
                html_document_a_project(file, output_dir)?;
                return Ok(());
            }
            if html {
                html_document_a_single_file(file)?;
            } else if json {
                json_document_a_single_file(file)?;
            }
            Ok(())
        }
        Command::Build { file, out } => {
            write_artifact_to_file(file, out, BuildFlag::SizeOptimized)?;
            Ok(())
        }
        Command::Repl => {
            start_repl();
            Ok(())
        }
    }
}
