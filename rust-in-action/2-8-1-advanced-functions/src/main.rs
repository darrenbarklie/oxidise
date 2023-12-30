fn main() {

    let a = 10;
    let b = 20;
    // refernce 10, 20
    let res = add_with_lifetimes(&a, &b);

    println!("{}", res);

    println!("Done!");
}

// `'a`: lifetime a
// `'b`: lifetime b
// `i: &'a i32`: parameter `i` is a reference to an `i32`
//                 with lifetime `a`
// `j: &'b i32`: parameter `j` is a reference to an `i32`
//                 with lifetime `b`

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
