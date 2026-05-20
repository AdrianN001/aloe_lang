use crate::test::util::test_cases_for_input_output;

#[test]
fn test_hashmap_new_methods() {
    let testcases = [
        ("let m = {\"a\":1}; m.get(\"b\", 5);", "5"),
        ("let m = {\"a\":1}; m.pop(\"a\");", "1"),
        ("let m = {\"a\":1}; m.pop(\"b\", 9);", "9"),
        ("let m = {}; m.setdefault(\"x\", 7); m.get(\"x\");", "7"),
        (
            "let m = {\"x\":5}; m.setdefault(\"x\", 7); m.get(\"x\");",
            "5",
        ),
        (
            "let a = {\"b\":2}; let m = {\"a\":1}; m.update(a); m.get(\"b\");",
            "2",
        ),
        (
            "let pairs = [[\"c\",3], [\"d\",4]]; let m = {}; m.update(pairs); m.get(\"c\");",
            "3",
        ),
    ];

    test_cases_for_input_output(&testcases);
}
