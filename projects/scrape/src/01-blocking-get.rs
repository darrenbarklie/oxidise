fn main() {
    blocking_get().unwrap();
}

fn blocking_get() -> Result<(), Box<dyn std::error::Error>> {
    let target_url = "https://www.rust-lang.org";
    let res = reqwest::blocking::get(target_url)?;

    let body = res.text()?;
    println!("{:?}", body);

    Ok(())
}
