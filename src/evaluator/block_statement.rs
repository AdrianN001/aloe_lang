use crate::object::Object;
use crate::ast::statement::block_statement::BlockStatement;
use crate::object::stack_environment::StackEnvironment;


impl BlockStatement{

    pub fn evaluate(&self, environ: &mut StackEnvironment) -> Result<Object, String>{

        let mut result = Object::NULL_OBJECT;

        for statement in self.statements.iter(){
            result = statement.evaluate(environ)?;

            if let Object::ReturnVal(_) = result{
                return Ok(result);
            }
        }


        Ok(result)
    }
}
