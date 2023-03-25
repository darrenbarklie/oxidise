fn main() {
    // let

    // let x = 6;
    // => error[E0384]: cannot assign twice to immutable variable `x`

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The new value of x is: {}", x);

    // const

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // shadowing

    let s = 2;
    let s = 2 + 1;

    {
        let s = s * 2;
        println!("The inner value of s is: {s} ");
    }

    println!("The outer value of s is: {s}");

    let spaces = "   ";
    let spaces = spaces.len();
}
