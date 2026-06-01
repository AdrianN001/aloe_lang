use crate::token::{
    Token,
    token_type::{TokenType, lookup_identifiers},
};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_pos: usize,

    character: char,

    pub current_line: usize,
}

impl Lexer {
    pub fn new(inp: String) -> Self {
        let mut new_lexer = Self {
            input: inp.chars().collect(),
            position: 0,
            read_pos: 0,

            character: 0 as char,
            current_line: 1,
        };

        new_lexer.read_char();

        new_lexer
    }

    fn skip_comment(&mut self) {
        while self.character != '\0' && self.character != '\n' {
            self.read_char();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.character.is_whitespace() || self.character == '#' {
            if self.character == '#' {
                self.skip_comment();
            }
            self.read_char();
        }
    }

    fn read_char(&mut self) -> Option<char> {
        if self.read_pos >= self.input.len() {
            self.character = 0 as char;
            self.position = self.read_pos;
            self.read_pos += 1;
            return None;
        }

        if self.character == '\n' {
            self.current_line += 1;
        }

        self.character = self.input[self.read_pos];
        self.position = self.read_pos;
        self.read_pos += 1;

        Some(self.character)
    }

    fn peek(&self) -> Option<char> {
        if self.read_pos >= self.input.len() {
            return None;
        }
        Some(self.input[self.read_pos])
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while is_identifier_part(self.character) {
            self.read_char();
        }

        self.input[start_pos..self.position]
            .iter()
            .copied()
            .collect()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;

        if self.character == '0' {
            if let Some(next_char) = self.peek() {
                let radix = match next_char {
                    'x' | 'X' => 16,
                    'b' | 'B' => 2,
                    'o' | 'O' => 8,
                    _ => 10,
                };

                if radix != 10 {
                    self.read_char(); // consume prefix indicator
                    self.read_char(); // move to first digit after prefix

                    let mut seen_digit = false;
                    while self.character == '_' || self.is_valid_digit_for(radix, self.character) {
                        if self.character != '_' {
                            seen_digit = true;
                        }
                        self.read_char();
                    }

                    if !seen_digit {
                        // Invalid prefix literal, keep the prefix intact and let parser handle the error.
                        self.step_back();
                    }

                    return self.input[start_pos..self.position]
                        .iter()
                        .copied()
                        .collect();
                }
            }
        }

        let mut last_was_underscore = false;

        while self.character.is_ascii_digit() || self.character == '_' {
            match self.character {
                '_' => {
                    if self.position == start_pos || last_was_underscore {
                        break;
                    }
                    last_was_underscore = true;
                }
                _ => {
                    last_was_underscore = false;
                }
            }
            self.read_char();
        }

        let mut end_pos = if last_was_underscore {
            self.position - 1
        } else {
            self.position
        };

        if start_pos == end_pos {
            end_pos += 1;
        }

        self.input[start_pos..end_pos]
            .iter()
            .filter(|c| **c != '_')
            .copied()
            .collect()
    }

    fn is_valid_digit_for(&self, radix: usize, character: char) -> bool {
        match radix {
            2 => character == '0' || character == '1',
            8 => character.is_ascii_digit() && character < '8',
            16 => character.is_ascii_hexdigit(),
            _ => false,
        }
    }

    fn step_back(&mut self) {
        if self.read_pos == 0 {
            return;
        }

        self.read_pos -= 1;
        self.position = self.read_pos;

        self.character = if self.position >= self.input.len() {
            '\0'
        } else {
            self.input[self.position]
        };
    }

    fn read_string(&mut self) -> String {
        self.read_char();

        let mut result = String::new();
        let mut brace_depth = 0;

        while self.character != '\0' {
            if self.character == '"' && brace_depth == 0 {
                break;
            }

            if self.character == '$' && self.peek() == Some('{') {
                brace_depth += 1;
                result.push(self.character);
                self.read_char();
                result.push(self.character);
                self.read_char();
                continue;
            }

            if brace_depth > 0 {
                if self.character == '{' {
                    brace_depth += 1;
                } else if self.character == '}' {
                    brace_depth -= 1;
                }
            } else if self.character == '\\' {
                self.read_char();
                if self.character == '\0' {
                    break;
                }
                let real_char = match self.character {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '"' => '"',
                    '\\' => '\\',
                    other => other,
                };
                result.push(real_char);
                self.read_char();
                continue;
            }

            result.push(self.character);
            self.read_char();
        }

        self.read_char();
        result
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let next_token = match self.character {
            '=' => {
                if let Some(next_char) = self.peek()
                    && next_char == '='
                {
                    self.read_char();
                    Token::simple(TokenType::Eq, "==", self.current_line)
                } else {
                    Token::simple(TokenType::Assign, "=", self.current_line)
                }
            }
            ';' => Token::simple(TokenType::Semicolon, ";", self.current_line),
            '(' => Token::simple(TokenType::LParen, "(", self.current_line),
            ')' => Token::simple(TokenType::RParen, ")", self.current_line),
            '[' => Token::simple(TokenType::LBracket, "[", self.current_line),
            ']' => Token::simple(TokenType::RBracket, "]", self.current_line),
            ',' => Token::simple(TokenType::Comma, ",", self.current_line),
            '+' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::PlusEq, "+=", self.current_line)
                }
                _ => Token::simple(TokenType::Plus, "+", self.current_line),
            },
            '-' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::MinusEq, "-=", self.current_line)
                }
                _ => Token::simple(TokenType::Minus, "-", self.current_line),
            },
            '/' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::DivEq, "/=", self.current_line)
                }
                _ => Token::simple(TokenType::Slash, "/", self.current_line),
            },
            '&' => match self.peek() {
                Some('&') => {
                    self.read_char();
                    Token::simple(TokenType::LogicalAnd, "&&", self.current_line)
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::BinaryAndEq, "&=", self.current_line)
                }
                _ => Token::simple(TokenType::BinaryAnd, "&", self.current_line),
            },
            '|' => match self.peek() {
                Some('|') => {
                    self.read_char();
                    Token::simple(TokenType::LogicalOr, "||", self.current_line)
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::BinaryOrEq, "|=", self.current_line)
                }
                _ => Token::simple(TokenType::BinaryOr, "|", self.current_line),
            },
            '^' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::BinaryXorEq, "^=", self.current_line)
                }
                _ => Token::simple(TokenType::LogicalXor, "^", self.current_line),
            },
            '*' => match self.peek() {
                Some('*') => {
                    self.read_char();
                    match self.peek() {
                        Some('=') => {
                            self.read_char();
                            Token::simple(TokenType::ExpoEq, "**=", self.current_line)
                        }
                        _ => Token::simple(TokenType::Exponent, "**", self.current_line),
                    }
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::MulEq, "*=", self.current_line)
                }
                _ => Token::simple(TokenType::Asterisk, "*", self.current_line),
            },
            '<' => match self.peek() {
                Some('-') => {
                    self.read_char();
                    Token::simple(TokenType::IteratorAssign, "<-", self.current_line)
                }
                Some('<') => {
                    self.read_char();
                    match self.peek() {
                        Some('=') => {
                            self.read_char();
                            Token::simple(TokenType::BinaryLeftShiftEq, "<<=", self.current_line)
                        }
                        _ => Token::simple(TokenType::BinaryLeftShift, "<<", self.current_line),
                    }
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::LE, "<=", self.current_line)
                }
                _ => Token::simple(TokenType::LT, "<", self.current_line),
            },
            '%' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::ModEq, "%=", self.current_line)
                }
                _ => Token::simple(TokenType::Modulo, "%", self.current_line),
            },
            '>' => match self.peek() {
                Some('>') => {
                    self.read_char();
                    match self.peek() {
                        Some('=') => {
                            self.read_char();
                            Token::simple(TokenType::BinaryRightShiftEq, ">>=", self.current_line)
                        }
                        _ => Token::simple(TokenType::BinaryRightShift, ">>", self.current_line),
                    }
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::GE, ">=", self.current_line)
                }
                _ => Token::simple(TokenType::GT, ">", self.current_line),
            },
            ':' => {
                if let Some(next_char) = self.peek()
                    && next_char == ':'
                {
                    self.read_char();
                    Token::simple(TokenType::ScopeResolution, "::", self.current_line)
                } else {
                    Token::simple(TokenType::Colon, ":", self.current_line)
                }
            }

            '.' => Token::simple(TokenType::Dot, ".", self.current_line),
            '!' => {
                if let Some(next_char) = self.peek()
                    && next_char == '='
                {
                    self.read_char();
                    Token::simple(TokenType::NotEq, "!=", self.current_line)
                } else {
                    Token::simple(TokenType::Bang, "!", self.current_line)
                }
            }
            '?' => {
                if let Some(next_char) = self.peek()
                    && next_char == '?'
                {
                    self.read_char();
                    Token::simple(TokenType::Coalescing, "??", self.current_line)
                } else {
                    Token::simple(TokenType::QuestionMark, "?", self.current_line)
                }
            }
            '{' => Token::simple(TokenType::LBrace, "{", self.current_line),
            '}' => Token::simple(TokenType::RBrace, "}", self.current_line),
            '"' => {
                let string_content = self.read_string();
                return Token::simple(TokenType::String, &string_content, self.current_line);
            }

            '\0' => Token::simple(TokenType::Eof, "", self.current_line),

            other_character => {
                if is_letter(other_character) {
                    let start = self.current_line;
                    let identifier = self.read_identifier();
                    let token_type = lookup_identifiers(&identifier);

                    return Token::new(token_type, identifier, start);
                } else if other_character.is_ascii_digit() {
                    let number = self.read_number();

                    return Token::new(TokenType::Integer, number, self.current_line);
                } else {
                    Token::new(
                        TokenType::Illegal,
                        self.character.to_string(),
                        self.current_line,
                    )
                }
            }
        };

        self.read_char();

        next_token
    }
}

fn is_letter(character: char) -> bool {
    character.is_ascii_lowercase() || character.is_ascii_uppercase() || character == '_'
}

fn is_identifier_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
