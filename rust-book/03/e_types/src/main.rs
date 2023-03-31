fn main() {
    // -------------------- //
    // --- SCALAR TYPES --- //
    // -------------------- //

    // --- Floating point type --- //

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // --- Numeric operations --- //

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // --- Boolean type --- //

    let t = true;
    let f: bool = false; // explicit type annotation

    // --- Character type --- //

    let c = 'z';
    let z: char = 'ℤ'; // explicit type
    let heart_eyed_cat = '😻';

    // ---------------------- //
    // --- COMPOUND TYPES --- //
    // ---------------------- //

    // --- Tuple type --- //

    let tup: (i32, f64, u8) = (500, 6.4, 1); // explicit type

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The values are: {five_hundred}, {six_point_four}, {one}");
}
