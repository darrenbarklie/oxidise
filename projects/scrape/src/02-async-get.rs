
// use reqwest::Client;
 
const URL_TARGET: &str = "https://craftapplied.com";

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  basic().await?;
  Ok(())
}

async fn basic() -> Result<(), Box<dyn std::error::Error>> {
  let body = reqwest::get(URL_TARGET).await?.text().await?;
  println!("{:?}", body);

  Ok(())
}