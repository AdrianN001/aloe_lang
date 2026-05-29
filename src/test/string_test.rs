use crate::test::util::test_cases_for_input_output;

#[test]
fn test_string_methods_strip_replace_slice() {
    let testcases = [
        ("\"  foo  \".strip()", "foo"),
        ("\"  bar\".lstrip()", "bar"),
        ("\"baz  \".rstrip()", "baz"),
        ("\"xxhelloxx\".strip(\"x\")", "hello"),
        ("\"abab\".replace(\"a\", \"x\")", "xbxb"),
        ("\"a a a\".replace(\"a\", \"b\", 1)", "b a a"),
        ("\"😀abçd\".slice(1,4)", "abç"),
        ("\"hello\".slice(-3,-1)", "ll"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_string_int_conversion() {
    let testcases = [
        ("40.as_str;", "40"),
        ("100.as_str(2)", "1100100"),
        ("100.as_str(16)", "64"),
        ("100.as_str(8)", "144"),
        ("100.as_str(-3)", "expected radix 2,8 or 16, got: '-3'"),
        ("\"40\".as_int", "40"),
        ("\"100\".as_int(2)", "4"),
        ("\"c0ffee\".as_int(16)", "12648430"),
        ("\"100\".as_int(-3)", "expected radix 2,8 or 16, got: '-3'"),
    ];

    test_cases_for_input_output(&testcases);
}
