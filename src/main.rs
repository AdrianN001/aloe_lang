use crate::cli::parse_cli;

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

fn main() {
    parse_cli();
}

#[cfg(test)]
mod test;
