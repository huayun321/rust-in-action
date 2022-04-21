fn main() {
    let mut arr = [1,2,3,4,5];

    for item in arr {
        // item += 1; cannot assign twice to immutable variable
        println!("item in arr {}", item);
    }

    for item in &arr {
        println!("item in &arr {}", item);
    }

    for item in &mut arr {
        *item += 1;
        println!("item in &mut arr {}", item);
    }

    println!("arr {:?}", arr);
}

