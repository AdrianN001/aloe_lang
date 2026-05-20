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
