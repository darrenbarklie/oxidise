fn main() {

    loop_with_values_owned();
    loop_with_values_referenced();
    loop_with_values_mutated();
    loop_anonymous();
    loop_with_index_value();

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

// Loop anonymous
fn loop_anonymous () {
    for _ in 0..6 {
        println!("Loop");
    }
}

// AVOID: Manually managing index value
//        Is valid Rust, but unperformant, unsafe
fn loop_with_index_value () {
    let collection = [1, 2, 3, 4, 5, 6];

    for i in 0..collection.len() {
        let item = collection[i];
        println!(item);
    }
}
