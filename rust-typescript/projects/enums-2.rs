struct Custom {
    age: usize,
    name: String,
}

// Enums can have SubTypes
// Added Type discrimination in a way in which there
// is no way to have type discrimination
enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello FEM".into()));
}

fn main() {
    let mut items: Vec<Item> = vec![];
    append(&mut items);
}
