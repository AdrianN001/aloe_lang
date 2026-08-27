use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_sys_module_import() {
    let testcases = [(
        "import {__pid} from \"@std::_sys\"; type(__pid());",
        "<type int>",
    )];

    test_cases_for_input_output(&testcases);
}
