use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_path() {
    let testcases = [
        (
            "
            import {_ntv} from \"@std::_ntv\";
            _ntv(\"Path\",\".\", \"extra_arg\");",
            "unexpected number of parameter for __path(). Expected: 1, got: '2'",
        ),
        (
            "
            import {_ntv} from \"@std::_ntv\";
            _ntv(\"Path\",23);",
            "unexpected parameter type for __path(). Expected: 'str', got: '<type int>'",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\".\")",
            "[PathWrapper for \".\"]",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\".\").exists;",
            "true",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\"not/existing/path\").exists;",
            "false",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\".\").parent();",
            "[PathWrapper for \"\"]",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\".\").as_absolute().exists;",
            "true",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\".\").is_dir;",
            "true",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\".\").is_file;",
            "false",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\"./Cargo.toml\").is_dir;",
            "false",
        ),
        (
            "
        import {_ntv} from \"@std::_ntv\";
        _ntv(\"Path\",\"./Cargo.toml\").is_file;",
            "true",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
pub fn test_ntv_module_import() {
    let testcases = [(
        "import {_ntv} from \"@std::_ntv\"; _ntv();",
        "expected a name for _ntv, got: 0 argument(s)",
    )];

    test_cases_for_input_output(&testcases);
}

#[test]
pub fn test_cmd() {
    let testcases = [
        (
            "
            import {_ntv} from \"@std::_ntv\";
        let command = _ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        command.program
        ",
            "echo",
        ),
        (
            "

            import {_ntv} from \"@std::_ntv\";
        let command = _ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        command.args
        ",
            "[hello, world]",
        ),
        (
            "

            import {_ntv} from \"@std::_ntv\";
        let command = _ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        status
        ",
            "0",
        ),
        (
            "
            import {_ntv} from \"@std::_ntv\";
        let command = _ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        stdout.as_str
        ",
            "hello, world\n",
        ),
        (
            "
            import {_ntv} from \"@std::_ntv\";
        let command = _ntv(\"Command\",\"echo\");
        command.add_arg(\"hello, world\");
        let [status, stdout, stderr] = command.output();
        stderr.as_str
        ",
            "",
        ),
    ];

    test_cases_for_input_output(&testcases);
}
