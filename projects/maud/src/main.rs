use maud::html;
use maud::PreEscaped;

fn main() {
    let name = "Darren";
    let markup = html! {
        p { "Hello " (name) "!" }
    };

    println!("{}", markup.into_string());


    let markup_esc_char = html! {
        "\"Oatmeal, are you crazy?\" he asked.\n"
        (PreEscaped("\"Oatmeal, are you crazy?\" he asked."))
    };

    println!("{}", markup_esc_char.into_string());

    let markup_esc_block = html! {
        pre {
            r#"
                Rocks, these are my rocks.
                Sediments make me sedimental.
                Smooth and round,
                Asleep in the ground.
                Shades of brown
                And gray.
            "#
        }
    };

    println!("{}", markup_esc_block.into_string());

    let document_object_model = dom();

    println!("{:?}", document_object_model);

    println!("Done!");
}

use maud::DOCTYPE;

fn dom () -> PreEscaped<String> {
    html! {
        (DOCTYPE)
        p { "Hello world!" }
    }
}
