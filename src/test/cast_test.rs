use crate::test::util::test_cases_for_input_output;

#[test]
fn test_int_ascii() {
    let testcases = [
        (r#""hello".as_byte_array();"#, "[104, 101, 108, 108, 111]"),
        ("104.as_utf_char();", "h"),
        (
            "10_000_000.as_utf_char();",
            "10000000 can not be converted to character.",
        ),
        (r#""".as_byte_array();"#, "[]"),
        (
            r#""hello".as_byte_array().map(fn(byte){byte.as_utf_char()}).join() == "hello";"#,
            "true",
        ),
        (r#" "😀".as_byte_array(); "#, "[240, 159, 152, 128]"),
        ("128512.as_utf_char();", "😀"),
    ];

    test_cases_for_input_output(&testcases);
}
