use crate::test::util::test_cases_for_input_output;

#[test]
fn test_async_await() {
    let testcases = [
        (
            "
async fun foo(){ return 42; }
__spawn((async fn(){let x = await foo(); print(x)
x;})())
",
            "null",
        ),
        (
            "
async fun a(){
    print(1);
    await __sleep2(10);
    print(2);
}

async fun b(){
    print(3);
}

__spawn(a());
__spawn(b());
",
            "1\n3\n2",
        ),
    ];

    test_cases_for_input_output(&testcases);
}
