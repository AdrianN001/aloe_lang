use std::env;

use crate::{repl::start_repl, script::run_script};

pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod repl;
pub mod script;
pub mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        start_repl();
    } else {
        run_script(&args[1]);
    }
}

#[cfg(test)]
mod eval_test;
#[cfg(test)]
mod lexer_test;
#[cfg(test)]
mod parser_test;
