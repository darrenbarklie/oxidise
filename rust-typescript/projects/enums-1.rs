enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }
    fn is_green_part(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Blue => return true,
            Color::Green => return false,
            Color::Yellow => return true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
    }
}

fn main() {
    let foo = Color::Green;
    println!("{}", foo.is_green());
    println!("{}", foo.is_green_part());
}
