use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_random_module_import() {
    let testcases = [(
        "import {_rnd} from \"@std::random\"; type(_rnd());",
        "<type float>",
    )];

    test_cases_for_input_output(&testcases);
}
