fn add_with_lifetimes<'a, 'b>(i:&'a i32, j:&'b i32) ->i32 {
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);

    println!("{}", res);
}

//Lifetime parameters are a way of providing control to the programmer
// while maintaining high-level code.