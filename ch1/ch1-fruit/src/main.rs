fn main() {
    let fruit = vec!['π₯', 'π', 'π'];
    let buffer_overflow = fruit[4];

    assert_eq!(buffer_overflow, 'π');
}

//ηΌε²ηΌθ―ζΊ’εΊζ£ζ΅
//thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', src/main.rs:3:27