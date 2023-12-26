fn main() {
    function_demo(2, 5);

    println!("Done!");
}

fn function_demo(n: i32, m: i32) -> i32 {
    let t = n + m;
    println!("Total: {}", t);
    t
}
