use crate::{lexer::Lexer, token::Token, token::token_type::TokenType};

#[test]
fn test_basic_lexer() {
    let input = "=+(){}[];";
    let mut lexer = Lexer::new(input.to_string());

    let expected_tokens = [
        Token::simple(TokenType::Assign, "=", 1),
        Token::simple(TokenType::Plus, "+", 1),
        Token::simple(TokenType::LParen, "(", 1),
        Token::simple(TokenType::RParen, ")", 1),
        Token::simple(TokenType::LBrace, "{", 1),
        Token::simple(TokenType::RBrace, "}", 1),
        Token::simple(TokenType::LBracket, "[", 1),
        Token::simple(TokenType::RBracket, "]", 1),
    ];

    expected_tokens.iter().for_each(|expected_token| {
        let lexer_token = &lexer.next_token();

        assert_eq!(expected_token, lexer_token);
    });
}

#[test]
fn test_basic_keywords_and_whitespaces_in_lexer() {
    let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
"foo bar"
"valami06"
[1,"asd"];
{"foo": "bar"};
string.length; 
for i <- range(0,20){ break true; }
for {}
{}.get()?
int.as_float()!
false ?? true
1 << 1
2 >> 1
+= -= *= /= %= **=
<<= >>= &= |= ^=
"#;
    let mut lexer = Lexer::new(input.to_string());

    let expected_tokens = [
        Token::simple(TokenType::KwLet, "let", 1),
        Token::simple(TokenType::Identifier, "five", 1),
        Token::simple(TokenType::Assign, "=", 1),
        Token::simple(TokenType::Integer, "5", 1),
        Token::simple(TokenType::Semicolon, ";", 1),
        Token::simple(TokenType::KwLet, "let", 2),
        Token::simple(TokenType::Identifier, "ten", 2),
        Token::simple(TokenType::Assign, "=", 2),
        Token::simple(TokenType::Integer, "10", 2),
        Token::simple(TokenType::Semicolon, ";", 2),
        Token::simple(TokenType::KwLet, "let", 3),
        Token::simple(TokenType::Identifier, "add", 3),
        Token::simple(TokenType::Assign, "=", 3),
        Token::simple(TokenType::KwFunction, "fn", 3),
        Token::simple(TokenType::LParen, "(", 3),
        Token::simple(TokenType::Identifier, "x", 3),
        Token::simple(TokenType::Comma, ",", 3),
        Token::simple(TokenType::Identifier, "y", 3),
        Token::simple(TokenType::RParen, ")", 3),
        Token::simple(TokenType::LBrace, "{", 3),
        Token::simple(TokenType::Identifier, "x", 4),
        Token::simple(TokenType::Plus, "+", 4),
        Token::simple(TokenType::Identifier, "y", 4),
        Token::simple(TokenType::Semicolon, ";", 4),
        Token::simple(TokenType::RBrace, "}", 5),
        Token::simple(TokenType::Semicolon, ";", 5),
        Token::simple(TokenType::KwLet, "let", 6),
        Token::simple(TokenType::Identifier, "result", 6),
        Token::simple(TokenType::Assign, "=", 6),
        Token::simple(TokenType::Identifier, "add", 6),
        Token::simple(TokenType::LParen, "(", 6),
        Token::simple(TokenType::Identifier, "five", 6),
        Token::simple(TokenType::Comma, ",", 6),
        Token::simple(TokenType::Identifier, "ten", 6),
        Token::simple(TokenType::RParen, ")", 6),
        Token::simple(TokenType::Semicolon, ";", 6),
        Token::simple(TokenType::String, "foo bar", 7),
        Token::simple(TokenType::String, "valami06", 8),
        Token::simple(TokenType::LBracket, "[", 9),
        Token::simple(TokenType::Integer, "1", 9),
        Token::simple(TokenType::Comma, ",", 9),
        Token::simple(TokenType::String, "asd", 9),
        Token::simple(TokenType::RBracket, "]", 9),
        Token::simple(TokenType::Semicolon, ";", 9),
        Token::simple(TokenType::LBrace, "{", 10),
        Token::simple(TokenType::String, "foo", 10),
        Token::simple(TokenType::Colon, ":", 10),
        Token::simple(TokenType::String, "bar", 10),
        Token::simple(TokenType::RBrace, "}", 10),
        Token::simple(TokenType::Semicolon, ";", 10),
        Token::simple(TokenType::Identifier, "string", 11),
        Token::simple(TokenType::Dot, ".", 11),
        Token::simple(TokenType::Identifier, "length", 11),
        Token::simple(TokenType::Semicolon, ";", 11),
        Token::simple(TokenType::KwFor, "for", 12),
        Token::simple(TokenType::Identifier, "i", 12),
        Token::simple(TokenType::IteratorAssign, "<-", 12),
        Token::simple(TokenType::Identifier, "range", 12),
        Token::simple(TokenType::LParen, "(", 12),
        Token::simple(TokenType::Integer, "0", 12),
        Token::simple(TokenType::Comma, ",", 12),
        Token::simple(TokenType::Integer, "20", 12),
        Token::simple(TokenType::RParen, ")", 12),
        Token::simple(TokenType::LBrace, "{", 12),
        Token::simple(TokenType::KwBreak, "break", 12),
        Token::simple(TokenType::KwTrue, "true", 12),
        Token::simple(TokenType::Semicolon, ";", 12),
        Token::simple(TokenType::RBrace, "}", 12),
        Token::simple(TokenType::KwFor, "for", 13),
        Token::simple(TokenType::LBrace, "{", 13),
        Token::simple(TokenType::RBrace, "}", 13),
        Token::simple(TokenType::LBrace, "{", 14),
        Token::simple(TokenType::RBrace, "}", 14),
        Token::simple(TokenType::Dot, ".", 14),
        Token::simple(TokenType::Identifier, "get", 14),
        Token::simple(TokenType::LParen, "(", 14),
        Token::simple(TokenType::RParen, ")", 14),
        Token::simple(TokenType::QuestionMark, "?", 14),
        Token::simple(TokenType::Identifier, "int", 15),
        Token::simple(TokenType::Dot, ".", 15),
        Token::simple(TokenType::Identifier, "as_float", 15),
        Token::simple(TokenType::LParen, "(", 15),
        Token::simple(TokenType::RParen, ")", 15),
        Token::simple(TokenType::Bang, "!", 15),
        Token::simple(TokenType::KwFalse, "false", 16),
        Token::simple(TokenType::Coalescing, "??", 16),
        Token::simple(TokenType::KwTrue, "true", 16),
        Token::simple(TokenType::Integer, "1", 17),
        Token::simple(TokenType::BinaryLeftShift, "<<", 17),
        Token::simple(TokenType::Integer, "1", 17),
        Token::simple(TokenType::Integer, "2", 18),
        Token::simple(TokenType::BinaryRightShift, ">>", 18),
        Token::simple(TokenType::Integer, "1", 18),
        Token::simple(TokenType::PlusEq, "+=", 19),
        Token::simple(TokenType::MinusEq, "-=", 19),
        Token::simple(TokenType::MulEq, "*=", 19),
        Token::simple(TokenType::DivEq, "/=", 19),
        Token::simple(TokenType::ModEq, "%=", 19),
        Token::simple(TokenType::ExpoEq, "**=", 19),
        Token::simple(TokenType::BinaryLeftShiftEq, "<<=", 20),
        Token::simple(TokenType::BinaryRightShiftEq, ">>=", 20),
        Token::simple(TokenType::BinaryAndEq, "&=", 20),
        Token::simple(TokenType::BinaryOrEq, "|=", 20),
        Token::simple(TokenType::BinaryXorEq, "^=", 20),
        Token::simple(TokenType::Eof, "", 20),
    ];

    expected_tokens.iter().for_each(|expected_token| {
        let lexer_token = &lexer.next_token();

        assert_eq!(expected_token, lexer_token);
    });
}

#[test]
fn test_integer_literal_bases_in_lexer() {
    let input = "0xdeadbeef 0b1011 0o77 0xDEADBEEF 0b1010_1010 0o7_7";
    let mut lexer = Lexer::new(input.to_string());

    let expected_tokens = [
        Token::simple(TokenType::Integer, "0xdeadbeef", 1),
        Token::simple(TokenType::Integer, "0b1011", 1),
        Token::simple(TokenType::Integer, "0o77", 1),
        Token::simple(TokenType::Integer, "0xDEADBEEF", 1),
        Token::simple(TokenType::Integer, "0b1010_1010", 1),
        Token::simple(TokenType::Integer, "0o7_7", 1),
        Token::simple(TokenType::Eof, "", 1),
    ];

    expected_tokens.iter().for_each(|expected_token| {
        let lexer_token = &lexer.next_token();

        assert_eq!(expected_token, lexer_token);
    });
}

#[test]
fn test_basic_lexer_with_eq_and_neq() {
    let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
return true;
} else {
return false;
}
10 == 10;
10 != 9;
let null_value = null;
launch main();";

    let mut lexer = Lexer::new(input.to_string());

    let expected_tokens = vec![
        // let five = 5;
        Token::simple(TokenType::KwLet, "let", 1),
        Token::simple(TokenType::Identifier, "five", 1),
        Token::simple(TokenType::Assign, "=", 1),
        Token::simple(TokenType::Integer, "5", 1),
        Token::simple(TokenType::Semicolon, ";", 1),
        // let ten = 10;
        Token::simple(TokenType::KwLet, "let", 2),
        Token::simple(TokenType::Identifier, "ten", 2),
        Token::simple(TokenType::Assign, "=", 2),
        Token::simple(TokenType::Integer, "10", 2),
        Token::simple(TokenType::Semicolon, ";", 2),
        // let add = fn(x, y) {
        Token::simple(TokenType::KwLet, "let", 3),
        Token::simple(TokenType::Identifier, "add", 3),
        Token::simple(TokenType::Assign, "=", 3),
        Token::simple(TokenType::KwFunction, "fn", 3),
        Token::simple(TokenType::LParen, "(", 3),
        Token::simple(TokenType::Identifier, "x", 3),
        Token::simple(TokenType::Comma, ",", 3),
        Token::simple(TokenType::Identifier, "y", 3),
        Token::simple(TokenType::RParen, ")", 3),
        Token::simple(TokenType::LBrace, "{", 3),
        // x + y;
        Token::simple(TokenType::Identifier, "x", 4),
        Token::simple(TokenType::Plus, "+", 4),
        Token::simple(TokenType::Identifier, "y", 4),
        Token::simple(TokenType::Semicolon, ";", 4),
        // };
        Token::simple(TokenType::RBrace, "}", 5),
        Token::simple(TokenType::Semicolon, ";", 5),
        // let result = add(five, ten);
        Token::simple(TokenType::KwLet, "let", 6),
        Token::simple(TokenType::Identifier, "result", 6),
        Token::simple(TokenType::Assign, "=", 6),
        Token::simple(TokenType::Identifier, "add", 6),
        Token::simple(TokenType::LParen, "(", 6),
        Token::simple(TokenType::Identifier, "five", 6),
        Token::simple(TokenType::Comma, ",", 6),
        Token::simple(TokenType::Identifier, "ten", 6),
        Token::simple(TokenType::RParen, ")", 6),
        Token::simple(TokenType::Semicolon, ";", 6),
        // !-/*5;
        Token::simple(TokenType::Bang, "!", 7),
        Token::simple(TokenType::Minus, "-", 7),
        Token::simple(TokenType::Slash, "/", 7),
        Token::simple(TokenType::Asterisk, "*", 7),
        Token::simple(TokenType::Integer, "5", 7),
        Token::simple(TokenType::Semicolon, ";", 7),
        // 5 < 10 > 5;
        Token::simple(TokenType::Integer, "5", 8),
        Token::simple(TokenType::LT, "<", 8),
        Token::simple(TokenType::Integer, "10", 8),
        Token::simple(TokenType::GT, ">", 8),
        Token::simple(TokenType::Integer, "5", 8),
        Token::simple(TokenType::Semicolon, ";", 8),
        // if (5 < 10) {
        Token::simple(TokenType::KwIf, "if", 9),
        Token::simple(TokenType::LParen, "(", 9),
        Token::simple(TokenType::Integer, "5", 9),
        Token::simple(TokenType::LT, "<", 9),
        Token::simple(TokenType::Integer, "10", 9),
        Token::simple(TokenType::RParen, ")", 9),
        Token::simple(TokenType::LBrace, "{", 9),
        // return true;
        Token::simple(TokenType::KwReturn, "return", 10),
        Token::simple(TokenType::KwTrue, "true", 10),
        Token::simple(TokenType::Semicolon, ";", 10),
        // } else {
        Token::simple(TokenType::RBrace, "}", 11),
        Token::simple(TokenType::KwElse, "else", 11),
        Token::simple(TokenType::LBrace, "{", 11),
        // return false;
        Token::simple(TokenType::KwReturn, "return", 12),
        Token::simple(TokenType::KwFalse, "false", 12),
        Token::simple(TokenType::Semicolon, ";", 12),
        // }
        Token::simple(TokenType::RBrace, "}", 13),
        // 10 == 10;
        Token::simple(TokenType::Integer, "10", 14),
        Token::simple(TokenType::Eq, "==", 14),
        Token::simple(TokenType::Integer, "10", 14),
        Token::simple(TokenType::Semicolon, ";", 14),
        // 10 != 9;
        Token::simple(TokenType::Integer, "10", 15),
        Token::simple(TokenType::NotEq, "!=", 15),
        Token::simple(TokenType::Integer, "9", 15),
        Token::simple(TokenType::Semicolon, ";", 15),
        // let nil_value = nil;
        Token::simple(TokenType::KwLet, "let", 16),
        Token::simple(TokenType::Identifier, "null_value", 16),
        Token::simple(TokenType::Assign, "=", 16),
        Token::simple(TokenType::KwNull, "null", 16),
        Token::simple(TokenType::Semicolon, ";", 16),
        // launch main();
        Token::simple(TokenType::KwLaunch, "launch", 17),
        Token::simple(TokenType::Identifier, "main", 17),
        Token::simple(TokenType::LParen, "(", 17),
        Token::simple(TokenType::RParen, ")", 17),
        Token::simple(TokenType::Semicolon, ";", 17),
        Token::simple(TokenType::Eof, "", 17),
    ];

    expected_tokens.iter().for_each(|expected_token| {
        let lexer_token = &lexer.next_token();

        assert_eq!(expected_token, lexer_token);
    });
}
