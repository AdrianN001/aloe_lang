use crate::{ast::{expression::identifier::Identifier, statement::block_statement::BlockStatement}, object::stack_environment::StackEnvironment};
use crate::ast::expression::function_expression::FunctionExpression;

#[derive(Clone, PartialEq, Eq)]
pub struct Function{
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement, 
    pub env:  StackEnvironment
}

impl Function{

    pub fn get_type(&self) -> String{
        "function".into()
    }

    pub fn inspect(&self) -> String{
        let mut buffer = String::new();

        buffer.push_str("fn");
        buffer.push('(');

        buffer.push_str(
            &self.parameters
                .iter()
                .map(|parameter| parameter.value.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );

        buffer.push_str(") {\n");
        buffer.push_str(&self.body.to_string());
        buffer.push_str("\n}");

        buffer
    }

    pub fn from_function_expression(expr: &FunctionExpression) -> Self{
        Self{
            parameters: expr.parameters.clone(),
            body:   expr.block.clone(),
            env: StackEnvironment::new()
        }
    }
}


