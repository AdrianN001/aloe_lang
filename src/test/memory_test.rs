use crate::test::util::test_cases_for_input_output;

#[test]
fn test_memory_builtin_functions() {
    let testcases = [
        (
            "
import {_id, _ref_n, _sizeof} from \"@std::_memory\";
let a = 1;
let b = a;
_id(a) == _id(b);",
            "true",
        ),
        (
            "

import {_id, _ref_n, _sizeof} from \"@std::_memory\";
let a = 1;
let b = a;
_ref_n(a) == _ref_n(b);",
            "true",
        ),
        (
            "

import {_id, _ref_n, _sizeof} from \"@std::_memory\";
let a = \"hello\";
let b = a;
_sizeof(a) == _sizeof(b);",
            "true",
        ),
        (
            "

import {_id, _ref_n, _sizeof} from \"@std::_memory\";
let a = \"hello\";
let f = fn(x){ _id(x) };
f(a) == _id(a);",
            "true",
        ),
        (
            "

import {_id, _ref_n, _sizeof} from \"@std::_memory\";
let a = 1;
let f = fn(x){ _ref_n(x) };
f(a) == 3;",
            "false", // int objects are copied, not shared, so ref count is 1 for each
        ),
        (
            "

import {_id, _ref_n, _sizeof} from \"@std::_memory\";
let a = 1;
let f = fn(x){ _sizeof(x) };
f(a) == _sizeof(a);",
            "true",
        ),
        (
            "

import {_id, _ref_n, _sizeof} from \"@std::_memory\";
            _ref_n();",
            "number_of_references expects exactly 1 argument",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_memory_module_import() {
    let testcases = [(
        "import {_id} from \"@std::_memory\"; type(_id(1));",
        "<type int>",
    )];

    test_cases_for_input_output(&testcases);
}
