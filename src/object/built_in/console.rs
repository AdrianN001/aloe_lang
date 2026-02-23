use crate::object::Object;

// print(object, ends)
pub fn console_write_builtin_function(args: &[Object]) -> Object {
    if args.is_empty() {
        return Object::NULL_OBJECT;
    }

    print!("{}", args[0].inspect());

    if args.len() > 1
        && let Object::String(end_str) = &args[1]
    {
        print!("{}", end_str.value);
    } else {
        println!();
    }
    Object::NULL_OBJECT
}
