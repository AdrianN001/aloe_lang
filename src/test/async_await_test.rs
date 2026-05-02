use crate::test::util::test_cases_for_input_output;

#[test]
fn test_async_await() {
    let testcases = [
        (
            "
async fun foo(){ return 42; }
__spawn((async fn(){let x = await foo(); let y = await foo(); x += y; print(x)
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
            "null",
        ),
        (
            "async fun main(){
        let file = __open(\"examples/math.aloe\");
        let content = await file.read_async()!;
        print(content);
    }
    __spawn(main());",
            "null",
        ),
    ];

    test_cases_for_input_output(&testcases);
}
