use crate::{ast::{program::Program, statement::{Statement, expression_statement::ExpressionStatement, return_statement::ReturnStatement}}, lexer::Lexer, token::{Token, token_type::TokenType}};
use crate::ast::precedence::OperationPrecedence;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::expression::identifier::Identifier;
use crate::ast::expression::Expression;

pub mod error;
pub mod expression;
pub mod statement;
pub mod program;
pub mod precedence;
pub mod expression_parse;

pub struct Parser{
    lexer:          Lexer,

    current_token:  Token,
    peek_token:     Token,

}

impl Parser{

    pub fn new(lexer: Lexer) -> Self{
        let mut new_parser = Self{
            lexer,
            current_token: Token::simple(TokenType::Illegal, "\0"),
            peek_token:    Token::simple(TokenType::Illegal, "\0"),
        };

        // both current_token und peek_token is set
        new_parser.next_token();
        new_parser.next_token();

        new_parser
    }

    fn next_token(&mut self){
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn into_a_program(mut self) -> Result<Program, String>{
        let mut program = Program::new();


        while self.current_token.token_type != TokenType::Eof{

            match self.parse_statement(){
                Ok(parsed_statement) => {
                    program.statements.push(parsed_statement);
                }
                Err(error_feedback) => {

                    println!("error: {}", error_feedback);
                    return Err(error_feedback);
                }
            }

            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, String>{
        match self.current_token.token_type{
            TokenType::KwLet => self.parse_let(),
            TokenType::KwReturn => self.parse_return(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Result<Statement, String>{
        let mut statement = LetStatement{
            token: self.current_token.clone(),
            name: Identifier::default(),
            value: Expression::default()
        };


        // if "let" is not followed by var name 
        if self.peek_token.token_type != TokenType::Identifier{
            return Err(self.create_unexpected_error_feedback(&TokenType::Identifier, &(self.peek_token.token_type)))
        }else{
            self.next_token();
        }

        statement.name = Identifier{
            value: self.current_token.literal.clone(), 
            token: self.current_token.clone()
        };

        if self.peek_token.token_type != TokenType::Assign{
            return Err(self.create_unexpected_error_feedback(&TokenType::Assign, &self.peek_token.token_type))
        }else{
            self.next_token();
        }


        while self.current_token.token_type != TokenType::Semicolon{
            self.next_token();
        }

        Ok(Statement::Let(statement))
    }

    fn parse_return(&mut self) -> Result<Statement, String>{
        let statement = ReturnStatement{
            token: self.current_token.clone(),
            value: Expression::default()
        };


        self.next_token();

        while self.current_token.token_type != TokenType::Semicolon{
            self.next_token();
        }


        Ok(Statement::Return(statement))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String>{
        let statement = ExpressionStatement{
            token: self.current_token.clone(),
            expression:  self.parse_expression(OperationPrecedence::Lowest)?
            
        };


        if self.peek_token.token_type == TokenType::Semicolon{
            self.next_token();
        }

        Ok(Statement::Expression(statement))
    }
}


