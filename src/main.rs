use serde_json::Value;

fn main() {
    println!("Look up the price of a cryptocurrency: ");
    let mut coin = String::new();

    let _ = std::io::stdin()
        .read_line(&mut coin)
        .expect("An error occurred while reading stdin");

    let res = get_price(&coin);

    match res {
        Ok(price) => println!("The price is: {price}"),
        Err(error) => println!("Error fetching the coin {error}"),
    }
}

fn get_price(coin: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{coin}"))
        .call()?
        .into_string()?;

    let parsed: Value = serde_json::from_str(&body).unwrap();
    Ok(parsed["market_data"]["current_price"]["usd"].to_string())
}
