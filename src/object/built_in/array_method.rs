use crate::object::{Object, array::Array, string_obj::StringObj};



pub fn rest_builtin_function(args: &[Object]) -> Object{
    if args.len() != 1{
        return Object::new_error(format!("expected 1 value, got {} value.", args.len()));
    }

    match &args[0]{
        Object::String(str) => {
            let new_string_value = if str.value.len() > 1{ 
                &str.value[1..] 
            } else {
                ""
            };

            Object::String(StringObj{value: new_string_value.into()})

        },
        Object::Array(arr) => {
            let new_array_value = if arr.items.len() > 1{
                &arr.items[1..]
            } else {
                &[]
            };

            Object::Array(Array{items: new_array_value.into()})
        },
        
        _ => Object::new_error(format!("unexpected argument type for len(): got {}", &args[0].get_type()))
    }
}


pub fn first_builtin_function(args: &[Object]) -> Object{
    if args.len() != 1{
        return Object::new_error(format!("expected 1 value, got {} value.", args.len()));
    }

    match &args[0]{
        Object::String(str) => {
            if !str.value.is_empty(){
                Object::String(StringObj{
                    value: str.value.chars().next().unwrap().to_string()
                })
            }else{
                Object::NULL_OBJECT
            }

        },
        Object::Array(arr) => {
            if !arr.items.is_empty(){
                arr.items[0].clone()
            }else{
                Object::NULL_OBJECT
            }        
        },
        
        _ => Object::new_error(format!("unexpected argument type for len(): got {}", &args[0].get_type()))
    }
}

pub fn last_builtin_function(args: &[Object]) -> Object{
    if args.len() != 1{
        return Object::new_error(format!("expected 1 value, got {} value.", args.len()));
    }

    match &args[0]{
        Object::String(str) => {
            if !str.value.is_empty(){
                Object::String(StringObj{
                    value: str.value.chars().next_back().unwrap().to_string()
                })
            }else{
                Object::NULL_OBJECT
            }

        },
        Object::Array(arr) => {
            if !arr.items.is_empty(){
                arr.items[arr.items.len()-1].clone()
            }else{
                Object::NULL_OBJECT
            }        
        },
        
        _ => Object::new_error(format!("unexpected argument type for len(): got {}", &args[0].get_type()))
    }
}

pub fn push_builtin_function(args: &[Object]) -> Object{
    if args.len() != 2{
        return Object::new_error(format!("expected 2 value, got {} value.", args.len()));
    }

    if !matches!(
        &args[0], 
        Object::Array(_)| Object::String(_)
        ){
        return Object::new_error(format!("expected the first value to be array or string, got {}.", args[0].get_type()));
    }

    match &args[0]{
        Object::String(str) => {
            if let Object::String(second_parameter_str) = &args[1]{
                return Object::String(StringObj{
                    value: str.value.clone() + &second_parameter_str.value
                });
            }
            Object::new_error("unmatching types: push(String, not String)".into())
        },
        Object::Array(arr) => {
            if let Object::Array(second_parameter_array) = &args[1]{
                return Object::Array(Array{
                    items: [&arr.items[..], &second_parameter_array.items[..]].concat()
                });
            }
            Object::Array(Array{
                items: {
                    let mut new_array = arr.items.clone();
                    new_array.push(args[1].clone());
                    new_array
                }
            })
        },
        
        _ => Object::new_error(format!("unexpected argument type for len(): got {}", &args[0].get_type()))
    }
}
