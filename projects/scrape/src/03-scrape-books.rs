use reqwest::Client;
use scraper::{Html, Selector};

// const URL_TARGET: &str = "https://craftapplied.com";
const URL_TARGET: &str = "http://books.toscrape.com/";

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

    let book_title_selector = Selector::parse("h3 > a").unwrap();
    for book_title in document.select(&book_title_selector) {
        let title = book_title.text().collect::<Vec<_>>();
        println!("Title: {}", title[0]);
    }

    let book_price_selector = Selector::parse(".price_color").unwrap();
    for book_price in document.select(&book_price_selector) {
        let price = book_price.text().collect::<Vec<_>>();
        println!("Price: {}", price[0]);
    }

    println!("---");

    Ok(())
}
