fn main() {

    loop_with_values_owned();
    loop_with_values_referenced();
    loop_with_values_mutated();

    println!("Done!");
}

// Loop with values owned
fn loop_with_values_owned () {
    let collection = [1, 2, 3, 4, 5, 6];

    for item in collection {
        println!("{}", item);
    }

    println!("---------");
}

// Loop with values referenced
fn loop_with_values_referenced () {
    let collection = [1, 2, 3, 4, 5, 6];

    for item in &collection {
        let item_doubled = item * 2;
        println!("{}", item_doubled);
    }

    println!("---------");
}

// Loop with values mutated
fn double(value: &mut i32) {
    *value *= 2;
}

fn loop_with_values_mutated () {
    let mut collection = [1, 2, 3, 4, 5, 6];
    
    for item in collection.iter_mut() {
        double(item);
        println!("{}", item);
    }

    println!("---------");
}
