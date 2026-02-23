use crate::object::{Object, integer::Integer};

pub fn len_builtin_function(args: &[Object]) -> Object {
    if args.len() != 1 {
        return Object::new_error(format!("expected 1 value, got {} value.", args.len()));
    }

    match &args[0] {
        Object::String(str) => Object::Int(Integer {
            value: str.value.len() as i64,
        }),
        Object::Array(arr) => Object::Int(Integer {
            value: arr.items.len() as i64,
        }),
        _ => Object::new_error(format!(
            "unexpected argument type for len(): got {}",
            &args[0].get_type()
        )),
    }
}
