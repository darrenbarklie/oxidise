fn main() {

    let mut grains: Vec<Cereal> = vec![];

    grains.push(Cereal::Rye);

    drop(grains);

    // Expect error: borrow of moved value: `grains`
    println!("{:?}", grains);

    println!("Done!");
}

#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}
