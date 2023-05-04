// Build: rustc line_reader.rs
// Run ./line_reader

fn main() {
    // Print every line from file
    let file = std::fs::read_to_string("./lines").unwrap();
    file.lines().for_each(|line| println!("{}", line));

    println!("---");

    // Print every other line
    let file = std::fs::read_to_string("./lines").unwrap();
    file.lines()
        .enumerate()
        .filter(|(i, _)| return i % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));

    println!("---");

    // Every other line
    // Skip two lines
    // Print two lines
    let file = std::fs::read_to_string("./lines").unwrap();
    file.lines()
        .enumerate()
        .filter(|(i, _)| return i % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));

    println!("---");
}
