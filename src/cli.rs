use clap::{Parser, Subcommand};

use crate::{
    doc::{html_document_a_single_file, json_document_a_single_file},
    repl::start_repl,
    script::run_script,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        file: Option<String>,
    },

    Doc {
        file: String,

        html: bool,

        #[arg(short, long)]
        json: bool,
    },

    Repl,
}

pub fn parse_cli() {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { file: file_opt } => match file_opt {
            Some(file) => {
                match run_script(&file) {
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                    Ok(_) => {}
                };
            }
            None => {
                //todo run main.aloe
            }
        },
        Command::Doc { file, html, json } => {
            if html {
                html_document_a_single_file(&file);
            } else if json {
                json_document_a_single_file(&file);
            }
        }
        Command::Repl => {
            start_repl();
        }
    }
}
