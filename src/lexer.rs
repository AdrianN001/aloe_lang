use crate::token::{
    Token,
    token_type::{TokenType, lookup_identifiers},
};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_pos: usize,

    character: char,
}

impl Lexer {
    pub fn new(inp: String) -> Self {
        let mut new_lexer = Self {
            input: inp.chars().collect(),
            position: 0,
            read_pos: 0,

            character: 0 as char,
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
                    Token::simple(TokenType::Eq, "==")
                } else {
                    Token::simple(TokenType::Assign, "=")
                }
            }
            ';' => Token::simple(TokenType::Semicolon, ";"),
            '(' => Token::simple(TokenType::LParen, "("),
            ')' => Token::simple(TokenType::RParen, ")"),
            '[' => Token::simple(TokenType::LBracket, "["),
            ']' => Token::simple(TokenType::RBracket, "]"),
            ',' => Token::simple(TokenType::Comma, ","),
            '+' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::PlusEq, "+=")
                }
                _ => Token::simple(TokenType::Plus, "+"),
            },
            '-' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::MinusEq, "-=")
                }
                _ => Token::simple(TokenType::Minus, "-"),
            },
            '/' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::DivEq, "/=")
                }
                _ => Token::simple(TokenType::Slash, "/"),
            },
            '&' => match self.peek() {
                Some('&') => {
                    self.read_char();
                    Token::simple(TokenType::LogicalAnd, "&&")
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::BinaryAndEq, "&=")
                }
                _ => Token::simple(TokenType::BinaryAnd, "&"),
            },
            '|' => match self.peek() {
                Some('|') => {
                    self.read_char();
                    Token::simple(TokenType::LogicalOr, "||")
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::BinaryOrEq, "|=")
                }
                _ => Token::simple(TokenType::BinaryOr, "|"),
            },
            '^' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::BinaryXorEq, "^=")
                }
                _ => Token::simple(TokenType::LogicalXor, "^"),
            },
            '*' => match self.peek() {
                Some('*') => {
                    self.read_char();
                    match self.peek() {
                        Some('=') => {
                            self.read_char();
                            Token::simple(TokenType::ExpoEq, "**=")
                        }
                        _ => Token::simple(TokenType::Exponent, "**"),
                    }
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::MulEq, "*=")
                }
                _ => Token::simple(TokenType::Asterisk, "*"),
            },
            '<' => match self.peek() {
                Some('-') => {
                    self.read_char();
                    Token::simple(TokenType::IteratorAssign, "<-")
                }
                Some('<') => {
                    self.read_char();
                    match self.peek() {
                        Some('=') => {
                            self.read_char();
                            Token::simple(TokenType::BinaryLeftShiftEq, "<<=")
                        }
                        _ => Token::simple(TokenType::BinaryLeftShift, "<<"),
                    }
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::LE, "<=")
                }
                _ => Token::simple(TokenType::LT, "<"),
            },
            '%' => match self.peek() {
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::ModEq, "%=")
                }
                _ => Token::simple(TokenType::Modulo, "%"),
            },
            '>' => match self.peek() {
                Some('>') => {
                    self.read_char();
                    match self.peek() {
                        Some('=') => {
                            self.read_char();
                            Token::simple(TokenType::BinaryRightShiftEq, ">>=")
                        }
                        _ => Token::simple(TokenType::BinaryRightShift, ">>"),
                    }
                }
                Some('=') => {
                    self.read_char();
                    Token::simple(TokenType::GE, ">=")
                }
                _ => Token::simple(TokenType::GT, ">"),
            },
            ':' => Token::simple(TokenType::Colon, ":"),

            '.' => Token::simple(TokenType::Dot, "."),
            '!' => {
                if let Some(next_char) = self.peek()
                    && next_char == '='
                {
                    self.read_char();
                    Token::simple(TokenType::NotEq, "!=")
                } else {
                    Token::simple(TokenType::Bang, "!")
                }
            }
            '?' => {
                if let Some(next_char) = self.peek()
                    && next_char == '?'
                {
                    self.read_char();
                    Token::simple(TokenType::Coalescing, "??")
                } else {
                    Token::simple(TokenType::QuestionMark, "?")
                }
            }
            '{' => Token::simple(TokenType::LBrace, "{"),
            '}' => Token::simple(TokenType::RBrace, "}"),
            '"' => {
                let string_content = self.read_string();
                return Token::simple(TokenType::String, &string_content);
            }

            '\0' => Token::simple(TokenType::Eof, ""),

            other_character => {
                if is_letter(other_character) {
                    let identifier = self.read_identifier();
                    let token_type = lookup_identifiers(&identifier);

                    return Token::new(token_type, identifier);
                } else if other_character.is_ascii_digit() {
                    let number = self.read_number();

                    return Token::new(TokenType::Integer, number);
                } else {
                    Token::new(TokenType::Illegal, self.character.to_string())
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
