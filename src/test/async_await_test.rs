use crate::test::util::test_cases_for_input_output;

#[test]
fn test_async_await() {
    let testcases = [
        (
            "
async fun foo(){ return 42; }
__spawn((async fn(){
    let y = await foo();
    println(y);
})())
",
            "null",
        ),
        (
            "
async fun foo(){ return 42; }
async fun bar(){ return 23; }
async fun add(x,y,z){ return x; }
async fun main(){
        let result = await add(await foo(),await foo(), await bar());
        println(result) 
    }
__spawn(main())",
            "null",
        ),
        (
            "
async fun change(x){ return !x; }
async fun main(){
        let result = await change(false);
        println(await change(false));
    }
__spawn(main())",
            "null",
        ),
        (
            "

async fun get_x(){ return 1; }
async fun get_y(){ let x = 23; return await get_x();} 
async fun get_z(){ return 23; }
async fun main(){
    let list = [await get_x(), await get_y(), await get_z(), get_z()];
    println(list)
    println(await list[-1])
    println(await list[-1])
    println(await get_x())
    println(await get_x())
    }
__spawn(main())",
            "null",
        ),
        (
            "

async fun get_list(){ return [1,2,3,4]; } 
async fun main(){
        println((await get_list())[(await get_list())[0]])
        println((await get_list())[3])

}
__spawn(main())",
            "null",
        ),
        (
            "

async fun get_list(){ return [1,2,3,4]; } 
async fun main(){
        if false{
            println(23);
        }

        let x = if false{
            println(23);
        }elif false{
            println(32);
            12
        }else{
            println(\"else block\");
        }

        println(x);
}
__spawn(main())",
            "null",
        ),
        (
            "
async fun get_number() { return 23; }
async fun get_string() { return \"test\"; }
async fun main(){
        let map = {};
        let map2 = {\"key\": \"value\", \"key2\": \"value2\"};
        let map3 = {(await get_number()) : (await get_string())};
        let map4 = {\"map\": map, \"map2\": map2, \"map3\": map3};
        println(map2);
        println(map3);
        println(map4);
}
__spawn(main())",
            "null",
        ), /*
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
           */
    ];

    test_cases_for_input_output(&testcases);
}
