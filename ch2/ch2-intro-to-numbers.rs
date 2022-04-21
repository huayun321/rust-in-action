fn main() {
    let twenty = 20; //编译器推断类型
    let twenty_one: i32 = 21; //变量类型注释
    let twenty_two= 22i32; //类型后缀

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000; //下划线提高阅读性，编译时会被编译器忽略
    println!("{}", one_million.pow(2));

    let forty_twos = [ //Creates an array of numbers, which must all be the same type, by surrounding those with square brackets
        42.0, //Floating-point literals without an explicit type annotation become 32-bit or 64-bit, depending on context.
        42f32,
        42.0_f32
    ];

    println!("{:02}", forty_twos[0]);
}

//Listing 2.3 Numeric literals and basic operations on numbers in Rust