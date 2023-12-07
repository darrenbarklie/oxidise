use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;
    let c: u16 = 1000;

    // promotion: safest to cast smaller type to larger one
    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }

    let c_ = c.try_into().unwrap();

    if a < c_ {
        println!("Ten is less than one thousand.");
    }

    println!("Hello, world!");
}
