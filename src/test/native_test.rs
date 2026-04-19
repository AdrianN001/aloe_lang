use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_path() {
    let testcases = [
        (
            "__path(\".\", \"extra_arg\");",
            "unexpected number of parameter for __path(). Expected: 1, got: '2'",
        ),
        (
            "__path(23);",
            "unexpected parameter type for __path(). Expected: 'str', got: 'integer'",
        ),
        ("__path(\".\")", "[PathWrapper for \".\"]"),
        ("__path(\".\").exists;", "true"),
        ("__path(\"not/existing/path\").exists;", "false"),
        ("__path(\".\").parent();", "[PathWrapper for \"\"]"),
        ("__path(\".\").as_absolute().exists;", "true"),
        ("__path(\".\").is_dir;", "true"),
        ("__path(\".\").is_file;", "false"),
        ("__path(\"./Cargo.toml\").is_dir;", "false"),
        ("__path(\"./Cargo.toml\").is_file;", "true"),
    ];

    test_cases_for_input_output(&testcases);
}
