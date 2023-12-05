fn main() {
    greet_world();
}

fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let greece = "Καλημέρα κόσμε";
    let japan = "ハロー・ワールド";
    
    let regions = [southern_germany, greece, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
