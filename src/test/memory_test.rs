use crate::test::util::test_cases_for_input_output;

#[test]
fn test_memory_builtin_functions() {
    let testcases = [
        (
            "
let a = 1;
let b = a;
id(a) == id(b);",
            "true",
        ),
        (
            "
let a = 1;
let b = a;
__ref_n(a) == __ref_n(b);",
            "true",
        ),
        (
            "
let a = \"hello\";
let b = a;
__sizeof(a) == __sizeof(b);",
            "true",
        ),
        (
            "
let a = \"hello\";
let f = fn(x){ id(x) };
f(a) == id(a);",
            "true",
        ),
        (
            "
let a = 1;
let f = fn(x){ __ref_n(x) };
f(a) == 3;",
            "false", // int objects are copied, not shared, so ref count is 1 for each
        ),
        (
            "
let a = 1;
let f = fn(x){ __sizeof(x) };
f(a) == __sizeof(a);",
            "true",
        ),
        (
            "__ref_n();",
            "number_of_references expects exactly 1 argument",
        ),
    ];

    test_cases_for_input_output(&testcases);
}
