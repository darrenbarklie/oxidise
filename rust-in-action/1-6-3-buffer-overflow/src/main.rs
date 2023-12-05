fn main() {
    let letters = vec!['a', 'b', 'c'];

    // expect error: main' panicked at src/main.rs:4:34:
    //   index out of bounds: the len is 3 but the index is 4 
    let buffer_overflow = letters[4];
    assert_eq!(buffer_overflow, 'd');

    println!("Done!");
}
