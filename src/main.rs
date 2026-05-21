use std::env;

use crate::{repl::start_repl, script::run_script};

pub mod ast;
pub mod evaluator;
pub mod frame;
pub mod lexer;
pub mod module;
pub mod object;
pub mod repl;
pub mod scheduler;
pub mod script;
pub mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        start_repl();
    } else {
        match run_script(&args[1]) {
            Err(error) => {
                eprintln!("{}", error);
            }
            Ok(_) => {}
        };
    }
}

#[cfg(test)]
mod test;
