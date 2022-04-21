fn main() {
    let mut arr = [1,2,3,4,5];

    //read
    for item in arr {
        // item += 1; cannot assign twice to immutable variable
        println!("item in arr {}", item);
    }
    println!();

    // read
    for item in &arr {
        println!("item in &arr {}", item);
    }
    println!();

    //read write
    for item in &mut arr {
        *item += 1;
        println!("item in &mut arr {}", item);
    }
    println!("arr {:?}", arr);
    println!();

    for item in 0..10 {
        println!("item in 0..10 {}", item);
    }
    println!();

    for item in 0..=10 {
        println!("item in 0..=10 {}", item);
    }
    println!();

    // bad performance
    // bad safety
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        println!("item in 0..collection.len() {}", item);
    }
    println!();

    for n in 0..10 {
        if n % 2 == 0 {
            println!("item in 0..10 continue {}", n);
            continue;
        }
    }

}

