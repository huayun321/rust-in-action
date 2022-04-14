fn main() {
    let fruit = vec!['🥝', '🍌', '🍓'];
    let buffer_overflow = fruit[4];

    assert_eq!(buffer_overflow, '🍉');
}

//缓冲编译溢出检测
//thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', src/main.rs:3:27