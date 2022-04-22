use std::ops::{Add};
use std::time::{Duration};

fn add<T: Add<Output = T>>(i:T, j:T) ->T { //The arguments to add() can accept any type that implements std::ops::Add.
    i + j
}

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(
        Duration::new(5, 0),
        Duration::new(10, 0)
    );

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}