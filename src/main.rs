use std::process::exit;

use serde::{Deserialize, Serialize};

fn main() {
    println!("Look up the price of a cryptocurrency: ");
    let mut coin = String::new();

    let _ = std::io::stdin()
        .read_line(&mut coin)
        .expect("An error occurred while reading stdin");

    match coin.trim_end() {
        "exit" => {
            println!("Bye!!");
            exit(3)
        }
        _ => {
            let res = get_price(&coin);

            match res {
                Ok(price) => {
                    println!("The price is: {price}");
                    main()
                }
                Err(error) => println!("Error fetching the coin {error}"),
            }
        }
    }
}

fn get_price(coin: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{coin}"))
        .call()?
        .into_string()?;

    let coin_data: Coin = serde_json::from_str(&body).unwrap();
    Ok(coin_data.market_data.current_price.usd.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct Coin {
    id: String,
    name: String,
    symbol: String,
    market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: Prices,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32,
    eur: f32,
}
