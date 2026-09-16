use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_sys_module_import() {
    let testcases = [(
        "import {_pid} from \"@std::_sys\"; type(_pid());",
        "<type int>",
    )];

    test_cases_for_input_output(&testcases);
}
