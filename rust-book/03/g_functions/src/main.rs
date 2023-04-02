fn main() {
    // Function One
    print_labeled_measurement(5, 'h');

    // Function Two
    let x = five();
    println!("The value of x is: {x}");

    // Function Three
    nested();
    // println!("The result of the nested function is: {n}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn f(x: i32) -> i32 {
    x + 1
}

fn nested() {
    println!(
        "Nested result is: {}",
        f({
            let y = 3;

            y + 1
        })
    );
}
