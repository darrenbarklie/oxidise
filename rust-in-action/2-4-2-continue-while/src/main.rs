fn main() {
    loop_continue();
    loop_while();
    println!("Done!");
}


// CONTINUE
fn loop_continue () {
    for n in 0..10 {
        if n % 2 == 0 {
            continue;
        }
        println!("{}", n);
    }
}

// WHILE
fn loop_while () {
    let mut counter = 0;

    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }
}
