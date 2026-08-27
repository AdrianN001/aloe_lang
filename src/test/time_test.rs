use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_time_module_import() {
    let testcases = [(
        "import {_time} from \"@std::_time\"; type(_time());",
        "<type int>",
    )];

    test_cases_for_input_output(&testcases);
}
