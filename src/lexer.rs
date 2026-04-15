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

    fn advance(&mut self) {
        self.read_pos += 1;
    }

    fn step_back(&mut self) {
        self.read_pos -= 1;
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.read_char() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(character) = self.read_char() {
            if character == '#' {
                self.skip_comment();
            } else if character.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_char(&mut self) -> Option<char> {
        if self.read_pos >= self.input.len() {
            self.character = 0 as char;
            return None;
        }

        self.character = self.input[self.read_pos];
        self.position = self.read_pos;

        Some(self.input[self.read_pos])
    }

    fn peek(&mut self) -> Option<char> {
        if self.read_pos + 1 >= self.input.len() {
            return None;
        }
        Some(self.input[self.read_pos + 1])
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        if let Some(current_char) = self.read_char() {
            if is_identifier_start(current_char) {
                self.advance();
            } else {
                return String::new();
            }
        }

        while let Some(current_char) = self.read_char() {
            if is_identifier_part(current_char) {
                self.advance();
            } else {
                self.step_back();
                break;
            }
        }
        self.input[start_pos..self.position]
            .iter()
            .copied()
            .collect()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;

        let mut last_was_underscore = false;

        while let Some(current_char) = self.read_char() {
            match current_char {
                c if c.is_ascii_digit() => {
                    last_was_underscore = false;
                    self.advance();
                }

                '_' => {
                    // darf nicht am Anfang oder doppelt sein
                    if self.position == start_pos || last_was_underscore {
                        break;
                    }

                    last_was_underscore = true;
                    self.advance();
                }

                _ => {
                    self.step_back();
                    break;
                }
            }
        }

        // darf nicht mit '_' enden
        let end_pos = if last_was_underscore {
            self.position - 1
        } else {
            self.position
        };

        // '_' entfernen
        self.input[start_pos..end_pos]
            .iter()
            .filter(|c| **c != '_')
            .copied()
            .collect()
    }

    fn read_string(&mut self) -> String {
        self.advance();
        self.read_char();

        let mut result = String::new();

        while let Some(current_char) = self.read_char() {
            match current_char {
                '\\' => {
                    // escape
                    self.advance();

                    if let Some(escaped_char) = self.read_char() {
                        let real_char = match escaped_char {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '"' => '"',
                            '\\' => '\\',
                            _ => escaped_char, // unknown escapes → 그대로
                        };
                        result.push(real_char);
                    }
                }
                '"' => break, // nur echtes Ende
                '\0' => break,
                _ => result.push(current_char),
            }
            self.advance();
        }

        result
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let next_token = match self.character {
            '=' => {
                if let Some(next_char) = self.peek()
                    && next_char == '='
                {
                    self.advance();
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
                    self.advance();
                    Token::simple(TokenType::PlusEq, "+=")
                }
                _ => Token::simple(TokenType::Plus, "+"),
            },
            '-' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Token::simple(TokenType::MinusEq, "-=")
                }
                _ => Token::simple(TokenType::Minus, "-"),
            },
            '/' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Token::simple(TokenType::DivEq, "/=")
                }
                _ => Token::simple(TokenType::Slash, "/"),
            },
            '&' => match self.peek() {
                Some('&') => {
                    self.advance();
                    Token::simple(TokenType::LogicalAnd, "&&")
                }
                Some('=') => {
                    self.advance();
                    Token::simple(TokenType::BinaryAndEq, "&=")
                }
                _ => Token::simple(TokenType::BinaryAnd, "&"),
            },
            '|' => match self.peek() {
                Some('|') => {
                    self.advance();
                    Token::simple(TokenType::LogicalOr, "||")
                }
                Some('=') => {
                    self.advance();
                    Token::simple(TokenType::BinaryOrEq, "|=")
                }
                _ => Token::simple(TokenType::BinaryOr, "|"),
            },
            '^' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Token::simple(TokenType::BinaryXorEq, "^=")
                }
                _ => Token::simple(TokenType::LogicalXor, "^"),
            },
            '*' => match self.peek() {
                Some('*') => {
                    self.advance();
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            Token::simple(TokenType::ExpoEq, "**=")
                        }
                        _ => Token::simple(TokenType::Exponent, "**"),
                    }
                }
                _ => Token::simple(TokenType::Asterisk, "*"),
            },
            '<' => match self.peek() {
                Some('-') => {
                    self.advance();
                    Token::simple(TokenType::IteratorAssign, "<-")
                }
                Some('<') => {
                    self.advance();
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            Token::simple(TokenType::BinaryLeftShiftEq, "<<=")
                        }
                        _ => Token::simple(TokenType::BinaryLeftShift, "<<"),
                    }
                }
                Some('=') => {
                    self.advance();
                    Token::simple(TokenType::LE, "<=")
                }
                _ => Token::simple(TokenType::LT, "<"),
            },
            '%' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Token::simple(TokenType::ModEq, "&=")
                }
                _ => Token::simple(TokenType::Modulo, "%"),
            },
            '>' => match self.peek() {
                Some('>') => {
                    self.advance();
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            Token::simple(TokenType::BinaryRightShiftEq, ">>=")
                        }
                        _ => Token::simple(TokenType::BinaryRightShift, ">>"),
                    }
                }
                Some('=') => {
                    self.advance();
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
                    self.advance();
                    Token::simple(TokenType::NotEq, "!=")
                } else {
                    Token::simple(TokenType::Bang, "!")
                }
            }
            '?' => {
                if let Some(next_char) = self.peek()
                    && next_char == '?'
                {
                    self.advance();
                    Token::simple(TokenType::Coalescing, "??")
                } else {
                    Token::simple(TokenType::QuestionMark, "?")
                }
            }
            '{' => Token::simple(TokenType::LBrace, "{"),
            '}' => Token::simple(TokenType::RBrace, "}"),
            '"' => {
                let string_content = self.read_string();

                Token::simple(TokenType::String, &string_content)
            }

            '\0' => Token::simple(TokenType::Eof, ""),

            other_character => {
                if is_letter(other_character) {
                    let identifier = self.read_identifier();
                    let token_type = lookup_identifiers(&identifier);

                    Token::new(token_type, identifier)
                } else if other_character.is_ascii_digit() {
                    let number = self.read_number();

                    Token::new(TokenType::Integer, number)
                } else {
                    Token::new(TokenType::Illegal, self.character.to_string())
                }
            }
        };

        self.read_char();
        self.advance();

        next_token
    }
}

fn is_letter(character: char) -> bool {
    character.is_ascii_lowercase() || character.is_ascii_uppercase() || character == '_'
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
