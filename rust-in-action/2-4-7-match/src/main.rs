fn main() {

    match_conditions();

    needle_haystack();

    println!("Done!");
}

fn match_conditions() {
    let n: i32 = 0;
    let n = 11;
    let n = 40;
    let n = 80;

    // Returns immediately when match found

    match n {
        0 => {
            println!("match single value, no operator");
        },
        10 ..= 20 => {
            println!("`..=` matches invlusive range");
        },
        40 | 80 => {
            println!("`|` matches either value");
        },
        _ => {
            println!("`_` matches every value");
        }
    }
}

fn needle_haystack() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }

}
