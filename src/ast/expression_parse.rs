
use crate::{ast::{Parser, expression::{Expression, integer_literal::IntegerLiteral, prefix_expression::PrefixExpression}, precedence::OperationPrecedence}, token::token_type::TokenType};
use crate::ast::expression::identifier::Identifier;


impl Parser{
    
    pub fn parse_expression(&mut self, prec: OperationPrecedence) -> Result<Expression, String>{
        let left_expression = match self.current_token.token_type{
            TokenType::Identifier => Ok(self.parse_identifier()),
            TokenType::Integer => self.parse_integer_literal(),
            TokenType::Bang |
            TokenType::Minus  =>  self.parse_prefix_expression(),
            _ => {
                return Err("".to_string());
            }
        };
        print!("");
        left_expression
    }

    fn parse_identifier(&self) -> Expression{
        Expression::Identifier(Identifier{
            token: self.current_token.clone(),
            value: self.current_token.literal.clone()
        })
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
}
