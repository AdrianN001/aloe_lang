use crate::{lexer::Lexer, token::Token, token::token_type::TokenType};

#[test]
fn test_basic_lexer() {
    let input = "=+(){}[];";
    let mut lexer = Lexer::new(input.to_string());

    let expected_tokens = [
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::Plus, "+"),
        Token::simple(TokenType::LParen, "("),
        Token::simple(TokenType::RParen, ")"),
        Token::simple(TokenType::LBrace, "{"),
        Token::simple(TokenType::RBrace, "}"),
        Token::simple(TokenType::LBracket, "["),
        Token::simple(TokenType::RBracket, "]"),
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
{"foo": "bar"}
"#;
    let mut lexer = Lexer::new(input.to_string());

    let expected_tokens = [
        Token::simple(TokenType::KwLet, "let"),
        Token::simple(TokenType::Identifier, "five"),
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::Integer, "5"),
        Token::simple(TokenType::Semicolon, ";"),
        Token::simple(TokenType::KwLet, "let"),
        Token::simple(TokenType::Identifier, "ten"),
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::Integer, "10"),
        Token::simple(TokenType::Semicolon, ";"),
        Token::simple(TokenType::KwLet, "let"),
        Token::simple(TokenType::Identifier, "add"),
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::KwFunction, "fn"),
        Token::simple(TokenType::LParen, "("),
        Token::simple(TokenType::Identifier, "x"),
        Token::simple(TokenType::Comma, ","),
        Token::simple(TokenType::Identifier, "y"),
        Token::simple(TokenType::RParen, ")"),
        Token::simple(TokenType::LBrace, "{"),
        Token::simple(TokenType::Identifier, "x"),
        Token::simple(TokenType::Plus, "+"),
        Token::simple(TokenType::Identifier, "y"),
        Token::simple(TokenType::Semicolon, ";"),
        Token::simple(TokenType::RBrace, "}"),
        Token::simple(TokenType::Semicolon, ";"),
        Token::simple(TokenType::KwLet, "let"),
        Token::simple(TokenType::Identifier, "result"),
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::Identifier, "add"),
        Token::simple(TokenType::LParen, "("),
        Token::simple(TokenType::Identifier, "five"),
        Token::simple(TokenType::Comma, ","),
        Token::simple(TokenType::Identifier, "ten"),
        Token::simple(TokenType::RParen, ")"),
        Token::simple(TokenType::Semicolon, ";"),
        Token::simple(TokenType::String, "foo bar"),
        Token::simple(TokenType::String, "valami06"),
        Token::simple(TokenType::LBracket, "["),
        Token::simple(TokenType::Integer, "1"),
        Token::simple(TokenType::Comma, ","),
        Token::simple(TokenType::String, "asd"),
        Token::simple(TokenType::RBracket, "]"),
        Token::simple(TokenType::Semicolon, ";"),
        Token::simple(TokenType::LBrace, "{"),
        Token::simple(TokenType::String, "foo"),
        Token::simple(TokenType::Colon, ":"),
        Token::simple(TokenType::String, "bar"),
        Token::simple(TokenType::RBrace, "}"),
        Token::simple(TokenType::Eof, ""),
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
10 != 9;";

    let mut lexer = Lexer::new(input.to_string());

    let expected_tokens = vec![
        // let five = 5;
        Token::simple(TokenType::KwLet, "let"),
        Token::simple(TokenType::Identifier, "five"),
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::Integer, "5"),
        Token::simple(TokenType::Semicolon, ";"),
        // let ten = 10;
        Token::simple(TokenType::KwLet, "let"),
        Token::simple(TokenType::Identifier, "ten"),
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::Integer, "10"),
        Token::simple(TokenType::Semicolon, ";"),
        // let add = fn(x, y) {
        Token::simple(TokenType::KwLet, "let"),
        Token::simple(TokenType::Identifier, "add"),
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::KwFunction, "fn"),
        Token::simple(TokenType::LParen, "("),
        Token::simple(TokenType::Identifier, "x"),
        Token::simple(TokenType::Comma, ","),
        Token::simple(TokenType::Identifier, "y"),
        Token::simple(TokenType::RParen, ")"),
        Token::simple(TokenType::LBrace, "{"),
        // x + y;
        Token::simple(TokenType::Identifier, "x"),
        Token::simple(TokenType::Plus, "+"),
        Token::simple(TokenType::Identifier, "y"),
        Token::simple(TokenType::Semicolon, ";"),
        // };
        Token::simple(TokenType::RBrace, "}"),
        Token::simple(TokenType::Semicolon, ";"),
        // let result = add(five, ten);
        Token::simple(TokenType::KwLet, "let"),
        Token::simple(TokenType::Identifier, "result"),
        Token::simple(TokenType::Assign, "="),
        Token::simple(TokenType::Identifier, "add"),
        Token::simple(TokenType::LParen, "("),
        Token::simple(TokenType::Identifier, "five"),
        Token::simple(TokenType::Comma, ","),
        Token::simple(TokenType::Identifier, "ten"),
        Token::simple(TokenType::RParen, ")"),
        Token::simple(TokenType::Semicolon, ";"),
        // !-/*5;
        Token::simple(TokenType::Bang, "!"),
        Token::simple(TokenType::Minus, "-"),
        Token::simple(TokenType::Slash, "/"),
        Token::simple(TokenType::Asterisk, "*"),
        Token::simple(TokenType::Integer, "5"),
        Token::simple(TokenType::Semicolon, ";"),
        // 5 < 10 > 5;
        Token::simple(TokenType::Integer, "5"),
        Token::simple(TokenType::LT, "<"),
        Token::simple(TokenType::Integer, "10"),
        Token::simple(TokenType::GT, ">"),
        Token::simple(TokenType::Integer, "5"),
        Token::simple(TokenType::Semicolon, ";"),
        // if (5 < 10) {
        Token::simple(TokenType::KwIf, "if"),
        Token::simple(TokenType::LParen, "("),
        Token::simple(TokenType::Integer, "5"),
        Token::simple(TokenType::LT, "<"),
        Token::simple(TokenType::Integer, "10"),
        Token::simple(TokenType::RParen, ")"),
        Token::simple(TokenType::LBrace, "{"),
        // return true;
        Token::simple(TokenType::KwReturn, "return"),
        Token::simple(TokenType::KwTrue, "true"),
        Token::simple(TokenType::Semicolon, ";"),
        // } else {
        Token::simple(TokenType::RBrace, "}"),
        Token::simple(TokenType::KwElse, "else"),
        Token::simple(TokenType::LBrace, "{"),
        // return false;
        Token::simple(TokenType::KwReturn, "return"),
        Token::simple(TokenType::KwFalse, "false"),
        Token::simple(TokenType::Semicolon, ";"),
        // }
        Token::simple(TokenType::RBrace, "}"),
        // 10 == 10;
        Token::simple(TokenType::Integer, "10"),
        Token::simple(TokenType::Eq, "=="),
        Token::simple(TokenType::Integer, "10"),
        Token::simple(TokenType::Semicolon, ";"),
        // 10 != 9;
        Token::simple(TokenType::Integer, "10"),
        Token::simple(TokenType::NotEq, "!="),
        Token::simple(TokenType::Integer, "9"),
        Token::simple(TokenType::Semicolon, ";"),
        Token::simple(TokenType::Eof, ""),
    ];

    expected_tokens.iter().for_each(|expected_token| {
        let lexer_token = &lexer.next_token();

        assert_eq!(expected_token, lexer_token);
    });
}
