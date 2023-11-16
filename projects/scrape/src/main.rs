use reqwest::Client;
use scraper::{Html, Selector};

use std::fs::File;
use std::io::Write;
use std::path::Path;

const URL_TARGET: &str = "https://www.craftapplied.com/";
const FILE_DEST: &str = "output/text.txt";

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    basic().await?;
    Ok(())
}

async fn basic() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(URL_TARGET).send().await.unwrap();
    let body = res.text().await.unwrap();
    let document = Html::parse_document(&body);

    // let property_url_selector = Selector::parse("#properties > li > a").unwrap();
    let property_url_selector = Selector::parse("body #work ul li h3").unwrap();

    for work_item in document.select(&property_url_selector) {
        let title = work_item.text().collect::<Vec<_>>();
        write_file(title);
    }

    println!("---");

    Ok(())
}

fn write_file(data: Vec<&str>) {
    let path = Path::new(FILE_DEST);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    for item in &data {
        match file.write_all(item.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}
