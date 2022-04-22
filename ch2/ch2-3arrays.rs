fn main() {
    let one = [1, 2, 3];
    let two:[u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // one[0] = 2;

    // The notation can be confusing.[T; n] describes an array’s type,
    // where T is the ele- ments’ type and n is a non-negative integer.
    // [f32; 12] denotes an array of 12 32-bit floating-point numbers.
    // It’s easy to get confused with slices [T], which do not have a compile-time length.

    // [u8; 3] is a different type than [u8; 4]. The size of the array matters to the type system.

    // Array indexing is bounds checked.

    let arrays = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}: ", a);

        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t(Σ{:?} = {})", a, sum);
    }
}