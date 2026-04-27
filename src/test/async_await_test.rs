use crate::test::util::test_cases_for_input_output;

#[test]
fn test_async_await() {
    let testcases = [(
        "
async fun a(){
    print(1);
    await sleep(10);
    print(2);
}

async fun b(){
    print(3);
}

__spawn(a());
__spawn(b());
",
        "null",
    )];

    test_cases_for_input_output(&testcases);
}
