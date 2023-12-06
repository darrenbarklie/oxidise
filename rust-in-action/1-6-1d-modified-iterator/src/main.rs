fn main() {
    let mut letters = vec![
        "a", "b", "c"
    ];

    for letter in letters {
        println!("{}", letter);
        // expect error:  borrow of moved value: `letters`
        letters.push(letter.clone());
    };

    println!("Done!");
}
