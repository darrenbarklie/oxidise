use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap();

    let search_term = "picture";

    let quote = "Every face, every shop, bedroom window, public
        house, and dark square is a picture feverishly tunred--
        in search of what? It is the same with books. What do
        we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            
            Some(_) => println!("{}", line),
            None => (),
        }
    }

    println!("Done!");
}
