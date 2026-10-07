use crate::{cli::parse_cli, signal_handling::handle_signal};

pub mod artifact;
pub mod ast;
pub mod cli;
pub mod doc;
pub mod evaluator;
pub mod frame;
pub mod lexer;
pub mod module;
pub mod object;
pub mod repl;
pub mod scheduler;
pub mod script;
pub mod symbol;
pub mod token;
pub mod version;

mod signal_handling;
pub mod terminal;

fn main() {
    handle_signal(parse_cli());
}

#[cfg(test)]
mod test;
