use crate::doc::doc_comment::parsed::ParsedDocComment;

#[test]
fn test_parse_from_function_with_empty_lists_and_example() {
    let raw = r#"str.length() -> int
returns the number of bytes in the string.
panics: []
errors: []
example:

---length.aloe---
let text = "Hello";
text.length(); # 5"#;

    let parsed = ParsedDocComment::parse_from_function(raw).expect("should be parsed");

    assert_eq!(
        parsed,
        ParsedDocComment::Function {
            declaration: String::from("str.length() -> int"),
            description: String::from("returns the number of bytes in the string."),
            panics: vec![],
            errors: vec![],
            example: Some(String::from(
                "---length.aloe---\nlet text = \"Hello\";\ntext.length(); # 5"
            )),
        }
    );
}

#[test]
fn test_parse_from_function_with_identifier_lists_and_no_example() {
    let raw = r#"str.as_float()? -> float
converts the string into a float.
panics: []
errors: [InvalidFloat]"#;

    let parsed = ParsedDocComment::parse_from_function(raw).expect("should be parsed");

    assert_eq!(
        parsed,
        ParsedDocComment::Function {
            declaration: String::from("str.as_float()? -> float"),
            description: String::from("converts the string into a float."),
            panics: vec![],
            errors: vec![String::from("InvalidFloat")],
            example: None,
        }
    );
}

#[test]
fn test_parse_from_function_with_multiple_identifiers_in_lists() {
    let raw = r#"buffer.get(index)
returns the byte at the given index.
panics: [WrongArgumentCount, WrongArgumentType]
errors: [IndexOutOfBounds]"#;

    let parsed = ParsedDocComment::parse_from_function(raw).expect("should be parsed");

    assert_eq!(
        parsed,
        ParsedDocComment::Function {
            declaration: String::from("buffer.get(index)"),
            description: String::from("returns the byte at the given index."),
            panics: vec![
                String::from("WrongArgumentCount"),
                String::from("WrongArgumentType")
            ],
            errors: vec![String::from("IndexOutOfBounds")],
            example: None,
        }
    );
}

#[test]
fn test_parse_from_function_with_multiline_description() {
    let raw = r#"str.reversed()
returns a reversed copy of the string.
this does not modify the original string.
panics: []
errors: []"#;

    let parsed = ParsedDocComment::parse_from_function(raw).expect("should be parsed");

    assert_eq!(
        parsed,
        ParsedDocComment::Function {
            declaration: String::from("str.reversed()"),
            description: String::from(
                "returns a reversed copy of the string.\nthis does not modify the original string."
            ),
            panics: vec![],
            errors: vec![],
            example: None,
        }
    );
}

#[test]
fn test_parse_from_function_without_panics_returns_none() {
    let raw = r#"str.length() -> int
returns the number of bytes in the string.
errors: []"#;

    assert_eq!(ParsedDocComment::parse_from_function(raw), None);
}

#[test]
fn test_parse_from_function_without_errors_returns_none() {
    let raw = r#"str.length() -> int
returns the number of bytes in the string.
panics: []"#;

    assert_eq!(ParsedDocComment::parse_from_function(raw), None);
}

#[test]
fn test_parse_from_function_with_non_list_panics_returns_none() {
    let raw = r#"str.length() -> int
returns the number of bytes in the string.
panics: 5
errors: []"#;

    assert_eq!(ParsedDocComment::parse_from_function(raw), None);
}

#[test]
fn test_parse_from_function_on_empty_string_returns_none() {
    assert_eq!(ParsedDocComment::parse_from_function(""), None);
}

#[test]
fn test_parse_from_struct_with_description_and_example() {
    let raw = r#"datatype: str
UTF-8 encoded immutable string.
example:

---str-literal.aloe---
let name = "Alice";
let empty = "";"#;

    let parsed = ParsedDocComment::parse_from_struct(raw).expect("should be parsed");

    assert_eq!(
        parsed,
        ParsedDocComment::Struct {
            description: String::from("datatype: str\nUTF-8 encoded immutable string."),
            example: Some(String::from(
                "---str-literal.aloe---\nlet name = \"Alice\";\nlet empty = \"\";"
            )),
        }
    );
}

#[test]
fn test_parse_from_struct_without_example() {
    let raw = r#"datatype: str
UTF-8 encoded immutable string."#;

    let parsed = ParsedDocComment::parse_from_struct(raw).expect("should be parsed");

    assert_eq!(
        parsed,
        ParsedDocComment::Struct {
            description: String::from("datatype: str\nUTF-8 encoded immutable string."),
            example: None,
        }
    );
}

#[test]
fn test_parse_from_struct_with_multiline_description() {
    let raw = r#"Person is a struct.
It has a name and an age."#;

    let parsed = ParsedDocComment::parse_from_struct(raw).expect("should be parsed");

    assert_eq!(
        parsed,
        ParsedDocComment::Struct {
            description: String::from("Person is a struct.\nIt has a name and an age."),
            example: None,
        }
    );
}

#[test]
fn test_parse_from_struct_on_empty_string_returns_none() {
    assert_eq!(ParsedDocComment::parse_from_struct(""), None);
}
