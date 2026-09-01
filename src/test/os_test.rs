use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_os_module_import() {
    #[cfg(target_os = "linux")]
    {
        let testcases = [(
            "import {__platform} from \"@std::_os\"; __platform();",
            "linux",
        )];

        test_cases_for_input_output(&testcases);
    }
}
