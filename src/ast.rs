use crate::{ast::{program::Program, statement::Statement}, lexer::Lexer, token::{Token, token_type::TokenType}};
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::expression::identifier::Identifier;
use crate::ast::expression::Expression;

pub mod expression;
pub mod statement;
pub mod program;


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
            peek_token:    Token::simple(TokenType::Illegal, "\0")
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

    pub fn into_a_program(mut self) -> Program{
        let mut program = Program::new();


        while self.current_token.token_type != TokenType::Eof{
           
            if let Some(parsed_statement) = self.parse_statement(){
                program.statements.push(parsed_statement); 
            }

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement>{
        match self.current_token.token_type{
            TokenType::KwLet => self.parse_let(),
            _ => None
        }
    }

    fn parse_let(&mut self) -> Option<Statement>{
        println!("{:?}", &self.current_token);
        let mut statement = LetStatement{
            token: self.current_token.clone(),
            name: Identifier::default(),
            value: Expression::default()
        };


        // if "let" is not followed by var name 
        if self.peek_token.token_type != TokenType::Identifier{
            return None;
        }else{
            self.next_token();
        }

        statement.name = Identifier{
            value: self.current_token.literal.clone(), 
            token: self.current_token.clone()
        };

        if self.peek_token.token_type != TokenType::Assign{
            return None;
        }else{
            self.next_token();
        }

        


        //TODO: We're skipping the expressions until we
// encounter a semicolon

        while self.current_token.token_type != TokenType::Semicolon{
            self.next_token();
        }

        Some(Statement::Let(statement))
    }
}


