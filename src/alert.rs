use play;
use reqwest::blocking::Client;
use reqwest::Error;
use serde_json::Value;
use std::io::Read;

/// BW API
const API: &str =
    "https://www.bw.com/exchange/config/controller/website/pricecontroller/getassistprice";

/// Function for check price
pub fn check(coin: String, price_str: String) {
    let body = get(String::from(API)).expect("Failed to get usd price");
    let json: Value = serde_json::from_str(&body).expect("Failed to parse json");

    let usd_str = &json["datas"]["usd"][&coin];
    let usd = usd_str
        .as_str()
        .unwrap()
        .parse::<f32>()
        .expect("Failed to parse string");

    let price = price_str.parse::<f32>().expect("Failed to parse string");

    if usd < price {
        println!("ALERT! PRICE {}: {}", coin.to_uppercase(), usd);
        play::play("baaah.mp3").expect("Failed to play mp3");
    }
}

/// Function for get requests
fn get(url: String) -> Result<String, Error> {
    let client = Client::new();
    let mut res = client.get(&url).send()?;
    let mut body = String::new();
    res.read_to_string(&mut body)
        .expect("Failed to read string");
    Ok(body)
}
