use crate::{ast::{expression::identifier::Identifier, statement::block_statement::BlockStatement}, object::{Object, stack_environment::StackEnvironment}};
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

    pub fn from_function_expression(expr: &FunctionExpression, environ: &StackEnvironment) -> Self{
        Self{
            parameters: expr.parameters.clone(),
            body:   expr.block.clone(),
            env: environ.clone()
        }
    }

    // Function calling 

    pub fn apply(&self, arguments: &[Object]) -> Result<Object, String>{

        let mut env = self.extend_environment_with_args(arguments);
        
        let last_expr = self.body.evaluate(&mut env)?;

        match last_expr{
            Object::ReturnVal(ret) => Ok(ret.unwrap_to_value()),
            other =>  Ok(other)
        }
    }

    fn extend_environment_with_args(&self, args: &[Object]) -> StackEnvironment{
        let mut new_env = StackEnvironment::new_enclosed(&self.env);

        self.parameters
            .iter()
            .enumerate()
            .for_each(|(indx, parameter)|{
                new_env.set(&parameter.value, args[indx].clone());
            });

        new_env
    }
}


