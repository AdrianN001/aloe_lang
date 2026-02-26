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

        while let Some(current_char) = self.read_char() {
            if is_letter(current_char) {
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

        while let Some(current_char) = self.read_char() {
            if current_char.is_ascii_digit() {
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

    fn read_string(&mut self) -> String {
        self.advance();
        self.read_char();

        let start_pos = self.position;

        while let Some(current_char) = self.read_char() {
            if current_char == '"' || current_char == '\0' {
                break;
            }

            self.advance();
        }

        self.input[start_pos..self.position]
            .iter()
            .copied()
            .collect()
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
            '+' => Token::simple(TokenType::Plus, "+"),
            '-' => Token::simple(TokenType::Minus, "-"),
            '/' => Token::simple(TokenType::Slash, "/"),
            '*' => Token::simple(TokenType::Asterisk, "*"),
            '<' => Token::simple(TokenType::LT, "<"),
            '>' => Token::simple(TokenType::GT, ">"),
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
