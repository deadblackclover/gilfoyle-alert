use play;
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::Read;

/// BW API
const API: &str =
    "https://www.bw.com/exchange/config/controller/website/pricecontroller/getassistprice";

/// Function for check price
pub fn check(coin: String, price_str: String) {
    let body = get(String::from(API));
    let json: Value = serde_json::from_str(&body).unwrap();

    let usd_str = &json["datas"]["usd"][&coin];
    let usd = usd_str.as_str().unwrap().parse::<f32>().unwrap();

    let price = price_str.parse::<f32>().unwrap();

    if usd < price {
        println!("ALERT! PRICE: {}", usd);
        play::play("baaah.mp3").unwrap();
    }
}

/// Function for get requests
fn get(url: String) -> String {
    let client = Client::new();
    let mut res = client.get(&url).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}
