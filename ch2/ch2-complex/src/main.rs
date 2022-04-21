use num::Complex; //The use keyword brings the Complex type into local scope.

fn main() {
    let a = Complex{ re: 2.1, im: -1.2}; //Every Rust type has a literal syntax.
    let b = Complex::new(11.1, 22.2); //Most types implement a new() static method.
    let result = a + b;

    println!("{} + {}i", result.re, result.im); //Accesses fields with the dot operator
}

//Listing 2.6 Calculating values with complex numbers