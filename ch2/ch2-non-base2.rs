fn main() {
    let three = 0b11; // 2**1 * 1 + 2**0 * 1
    let thirty = 0o36; // 8**1 * 3 + 8**0 * 6
    let three_hundred = 0x12C; // 16**2 * 1 + 16**1 * 2 + 16**0 * 12

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

//Listing 2.4 Using base 2, base 8, and base 16 numeric literals
//不同进制数字字面量