use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_os_module_import() {
    #[cfg(target_os = "linux")]
    {
        let testcases = [(
            "import {_platform} from \"@std::_os\"; _platform();",
            "linux",
        )];

        test_cases_for_input_output(&testcases);
    }
}
