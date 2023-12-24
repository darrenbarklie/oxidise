fn main() {

    looper_simple();

    looper_nested();

    println!("And done!");
}

fn looper_simple() {
    let mut n = 0;
    
    loop {
        println!("Loop: {}", n);
        let m = n;
        n = m + 1;

        if n == 1_000_000 {
            println!("Done!");
            break;
        }
    }
}

// Use loop labels
fn looper_nested() {
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {

                println!("---");
                println!("x: {}, y: {}, z: {}", &x, &y, z);

                if x + y + z > 1_000 {
                    break 'outer;
                }
            }
        }
    }
}
