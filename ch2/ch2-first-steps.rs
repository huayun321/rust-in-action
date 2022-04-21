fn main() {
    let a = 10;
    let b:i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (d + d) = {}", e);
}

fn add(i:i32, j:i32) -> i32 {
    i + j  //注意没有分号
}

//变量声明和函数调用
//可以由编译器根据数字字面量去判断
//也可以放在变量后
//或者放在字面量中