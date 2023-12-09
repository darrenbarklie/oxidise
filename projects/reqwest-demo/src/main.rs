use reqwest::{blocking::Client, blocking::ClientBuilder, redirect::Policy};
fn main() {
    let http_client = Client::new();

    // GET
    let http_result = http_client.get("https://daz.dev").send();

    if http_result.is_ok() {
        println!("{:#?}", http_result.ok().unwrap().text().unwrap());
    } else if http_result.is_err() {
        println!("Error occurred: {:#?}", http_result.err());
    }

    println!("-------------------------");

    // POST
    let post_result = http_client
        .post("http://localhost:3000/send_data")
        .header("User-Agent", "Rust Reqwest Demo Application")
        .body("{\"first_name\": \"Daz\"}")
        .send();

    println!("{:#?}", post_result.ok().unwrap().text().unwrap());

    println!("-------------------------");

    // REDIRECT
    let redir_policy = Policy::limited(0);
    let http_client = ClientBuilder::new()
        .redirect(redir_policy)
        .build()
        .ok()
        .unwrap();
    let http_result = http_client.get("http://localhost:3000/redirect_one").send();

    match http_result {
        Ok(res) => {
            println!("Received API response: {:#?}", res.text().unwrap());
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    println!("-------------------------");

    println!("Done!");
}
