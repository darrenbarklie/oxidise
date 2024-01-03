fn main() {

    let result = add(2, 3);
    println!("{}", result);

    println!("Done!");
}

// Capital letters indidcate generic type [T, U, V]
// E used as generic error type

// expect error: as no trait bound defined
// fn add<T>(i: T, j: T) -> T {

// Traits enable types to advertise that they are using
// common behaviour
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
