fn main() {
    let abc:(f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz:(f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("      0.1 + 0.2 : {:b}", (abc.0 + abc.1).to_bits());
    println!("            0.3 : {:b}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("      0.1 + 0.2 : {:b}", (xyz.0 + xyz.1).to_bits());
    println!("            0.3 : {:b}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2); //Runs successfully
    assert!(xyz.0 + xyz.1 == xyz.2); //Triggers a crash
}
//These often approximate the numbers that they’re representing. Floating-point types are implemented in base 2,
// but we often want to calculate numbers in base 10.

// Avoid testing floating-point numbers for equality.
// Be wary when results may be mathematically undefined.

//
// fn main() {
//     let result: f32 = 0.1 + 0.1;
//     let desired: f32 = 0.2;
//     let absolute_difference = (desired - result).abs();
//     assert!(absolute_difference <= f32::EPSILON);
// }
// 可以判断是否在范围内

// NAN values poison other numbers. Almost all operations interacting with NAN return NAN.
// Another thing to be mindful of is that, by definition, NAN values are never equal.
// This small program will always crash:
// fn main() {
//     let x = (-42.0_f32).sqrt();
//     assert_eq!(x, x);
// }
//
// make use of the is_nan() and is_finite() methods.
// fn main() {
//     let x: f32 = 1.0 / 0.0;
//     assert!(x.is_finite());
// }
//

