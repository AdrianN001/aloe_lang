use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_path() {
    let testcases = [
        (
            "__ntv(\"Path\",\".\", \"extra_arg\");",
            "unexpected number of parameter for __path(). Expected: 1, got: '2'",
        ),
        (
            "__ntv(\"Path\",23);",
            "unexpected parameter type for __path(). Expected: 'str', got: '<type int>'",
        ),
        ("__ntv(\"Path\",\".\")", "[PathWrapper for \".\"]"),
        ("__ntv(\"Path\",\".\").exists;", "true"),
        ("__ntv(\"Path\",\"not/existing/path\").exists;", "false"),
        ("__ntv(\"Path\",\".\").parent();", "[PathWrapper for \"\"]"),
        ("__ntv(\"Path\",\".\").as_absolute().exists;", "true"),
        ("__ntv(\"Path\",\".\").is_dir;", "true"),
        ("__ntv(\"Path\",\".\").is_file;", "false"),
        ("__ntv(\"Path\",\"./Cargo.toml\").is_dir;", "false"),
        ("__ntv(\"Path\",\"./Cargo.toml\").is_file;", "true"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
pub fn test_cmd() {
    let testcases = [
        (
            "
        let command = __ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        command.program
        ",
            "echo",
        ),
        (
            "
        let command = __ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        command.args
        ",
            "[hello, world]",
        ),
        (
            "
        let command = __ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        status
        ",
            "0",
        ),
        (
            "
        let command = __ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        stdout.as_str
        ",
            "hello, world\n",
        ),
        (
            "
        let command = __ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        stderr.as_str
        ",
            "",
        ),
    ];

    test_cases_for_input_output(&testcases);
}
