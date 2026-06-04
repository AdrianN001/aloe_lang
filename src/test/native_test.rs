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
            "unexpected parameter type for __path(). Expected: 'str', got: '<type int>'",
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

#[test]
pub fn test_cmd() {
    let testcases = [
        (
            "
        let command = __cmd(\"echo\");
        command.add_arg(\"hello, world\");
        command.program
        ",
            "echo",
        ),
        (
            "
        let command = __cmd(\"echo\");
        command.add_arg(\"hello, world\");
        command.args
        ",
            "[hello, world]",
        ),
        (
            "
        let command = __cmd(\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        status
        ",
            "0",
        ),
        (
            "
        let command = __cmd(\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        stdout.as_str
        ",
            "hello, world\n",
        ),
        (
            "
        let command = __cmd(\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        stderr.as_str
        ",
            "",
        ),
    ];

    test_cases_for_input_output(&testcases);
}
