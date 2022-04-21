fn main() {
    // let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item { //This match expression returns a value that can be bound to a variable.
            42 | 132 => "hit!", //Success! 42 | 132 matches  42 or 132.
            _ => "miss", //A wildcard pattern that matches everything
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}