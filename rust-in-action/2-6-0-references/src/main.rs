fn main() {
    let a = 42;

    let r = &a;         // referencing

    let b = a + *r;     // dereferencing

    println!("a + a = {}", b);

    let needle = 21147;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }

    println!("Done!");
}
