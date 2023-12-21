use maud::html;

fn main() {
    let name = "Darren";
    let markup = html! {
        p { "Hello " (name) "!" }
    };

    println!("{}", markup.into_string());

    println!("Done!");
}
