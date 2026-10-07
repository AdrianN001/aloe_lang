use std::println;

use crate::terminal::formatter::TerminalTextFormatter;

pub struct TerminalHandler;

impl TerminalHandler {
    pub fn write(msg: &str) {
        print!("{}", msg);
    }
    pub fn writeln(msg: &str) {
        println!("{}", msg);
    }

    pub fn write_error(msg: &str) {
        eprintln!("{}", msg);
    }

    pub fn flush() {
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }

    // helper methods
    pub fn writeln_bold(msg: &str) {
        let formatted_msg = TerminalTextFormatter::to_bold(msg);
        Self::writeln(&formatted_msg);
    }
    pub fn writeln_italic(msg: &str) {
        let formatted_msg = TerminalTextFormatter::to_italic(msg);
        Self::writeln(&formatted_msg);
    }
    pub fn writeln_underlined(msg: &str) {
        let formatted_msg = TerminalTextFormatter::to_underlined(msg);
        Self::writeln(&formatted_msg);
    }

    pub fn writeln_error(msg: &str) {
        Self::write_error(msg);
    }

    pub fn read() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input
    }
}
