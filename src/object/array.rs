use crate::object::Object;



#[derive(PartialEq, Eq, Clone)]
pub struct Array{
    pub items: Vec<Object>
}


impl Array{
    pub fn get_type(&self) -> String{
        "array".into()
    }

    pub fn inspect(&self) -> String{
        let mut buffer = String::new();

        buffer.push('[');

        buffer.push_str(
            &self.items
                .iter()
                .map(|item| item.inspect() )
                .collect::<Vec<_>>()
                .join(", ")
        );

        buffer.push(']');
        
        buffer
    }

 
}
