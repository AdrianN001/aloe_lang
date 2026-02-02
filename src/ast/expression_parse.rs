
use crate::{ast::{Parser, expression::{Expression, if_expression::IfExpression, infix::InfixExpression, integer_literal::IntegerLiteral, prefix_expression::PrefixExpression}, precedence::{OperationPrecedence, get_precedence_of_operator}, statement::block_statement::BlockStatement}, token::token_type::TokenType};
use crate::ast::expression::boolean::Boolean;
use crate::ast::expression::identifier::Identifier;


impl Parser{
    
    pub fn parse_expression(&mut self, prec: OperationPrecedence) -> Result<Expression, String>{
        let mut left_expression = match self.current_token.token_type{
            TokenType::Identifier => Ok(self.parse_identifier()),
            TokenType::Integer => self.parse_integer_literal(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::KwIf => self.parse_if_expression(),
            
            TokenType::Bang |
            TokenType::Minus  =>  self.parse_prefix_expression(),

            TokenType::KwTrue |
                TokenType::KwFalse => Ok(self.parse_boolean()),
            _ => {
                return Err(format!("no prefix function for {} found", &self.current_token.token_type));
            }
        }.unwrap();

        while self.peek_token.token_type != TokenType::Semicolon && prec < get_precedence_of_operator(&self.peek_token){
            match self.peek_token.token_type{
                TokenType::Plus | TokenType::Minus | TokenType::Slash |
                    TokenType::Asterisk | TokenType::Eq | TokenType::NotEq | 
                    TokenType::LT | TokenType::GT => {
                        self.next_token();
                        left_expression = self.parse_infix_expression(&left_expression)?;
                    }
                    _ => {
                        return Ok(left_expression);  
                    }
            };
        }

        Ok(left_expression)
    }

    fn parse_identifier(&self) -> Expression{
        Expression::Identifier(Identifier{
            token: self.current_token.clone(),
            value: self.current_token.literal.clone()
        })
    }

    fn parse_boolean(&self) -> Expression{
        Expression::Bool(Boolean{
            token: self.current_token.clone(),
            value: self.current_token.token_type == TokenType::KwTrue
        })
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, String>{
        self.next_token();

        let expression = self.parse_expression(OperationPrecedence::Lowest);
        if self.peek_token.token_type == TokenType::RParen{
            return Err("expected 'expression', got 'RParen'".to_string())
        }

        expression
    }
    fn parse_integer_literal(&self) -> Result<Expression, String>{
        match self.current_token.literal.parse::<i64>(){
            Ok(integer_value) => Ok(
                Expression::IntegerLiteral(
                    IntegerLiteral {
                        token: self.current_token.clone(), 
                        value: integer_value 
                    })),
            Err(_) => Err(format!("could convert int literal \"{}\" to int", &self.current_token.literal))
        } 
    } 

    fn parse_prefix_expression(&mut self) -> Result<Expression, String>{
        let mut expression = PrefixExpression{
            token: self.current_token.clone(),
            operator: self.current_token.literal.clone(),
            right: Box::new(Expression::default())
        };

        self.next_token();

        let right_from_prefix = match self.parse_expression(OperationPrecedence::Prefix){
            Ok(ok) => ok,
            Err(error_feedback) => {
                return Err(error_feedback);
            }
        };

        expression.right = Box::new(right_from_prefix);

        Ok(Expression::Prefix(expression))
    }

    fn parse_infix_expression(&mut self, left: &Expression) -> Result<Expression, String>{

        let infix_expression = InfixExpression{
            token: self.current_token.clone(), 
            operator: self.current_token.literal.clone(),
            left: Box::new(left.to_owned()),
            right: {
                let precedence = get_precedence_of_operator(&self.current_token);
                self.next_token();
                match self.parse_expression(precedence){
                    Ok(expr) => Box::new(expr),
                    Err(error) => return Err(error)
                }
            }
        };


        Ok(Expression::Infix(infix_expression))
    } 

    fn parse_if_expression(&mut self) -> Result<Expression, String>{
        let mut expr = IfExpression{
            token: self.current_token.clone(),
            ..Default::default()
        };

        if self.peek_token.token_type != TokenType::LParen{

            return Err("unexpected token. Expected 'LParen'".to_string());
        }
        self.next_token();

        self.next_token();
        expr.condition = match self.parse_expression(OperationPrecedence::Lowest){
            Ok(valid_expr) => Box::new(valid_expr),
            Err(error_feedback) => return Err(error_feedback)
        };

        if self.peek_token.token_type != TokenType::RParen{
            return Err("unexpected token: Expected 'RParen'".to_string());
        }
        self.next_token();

        if self.peek_token.token_type != TokenType::LBrace{
            return Err("unexpected token: Expected 'LBrace'".to_string());
        }
        self.next_token();
        
        expr.consequence = self.parse_block_statement()?;

        if self.peek_token.token_type == TokenType::KwElse{
            self.next_token();

            if self.peek_token.token_type != TokenType::LBrace{
                return Err("unexpected token: Expected 'LBrace'".to_string());
            }

            expr.alternative = match self.parse_block_statement(){
                Ok(block_statement) => Some(block_statement),
                Err(error_feedback) => return Err(error_feedback)
            };
        }

        Ok(Expression::If(expr))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, String>{
        let mut block = BlockStatement{
            token: self.current_token.clone(),
            statements: Vec::new()
        };

        self.next_token();

        while self.current_token.token_type != TokenType::RBrace && self.current_token.token_type != TokenType::Eof{
            let statement = self.parse_statement();
            
            match statement{
                Ok(valid_statement) => block.statements.push(valid_statement),
                Err(error_feedback) => return Err(error_feedback)
            }
            self.next_token();
        }

        Ok(block)
    }
}






