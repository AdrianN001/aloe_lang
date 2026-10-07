pub struct TerminalTextFormatter;

impl TerminalTextFormatter {
    pub fn to_bold(msg: &str) -> String {
        format!("\x1b[1m{}\x1b[0m", msg)
    }
    pub fn to_italic(msg: &str) -> String {
        format!("\x1b[3m{}\x1b[0m", msg)
    }
    pub fn to_underlined(msg: &str) -> String {
        format!("\x1b[4m{}\x1b[0m", msg)
    }
}
