use clap::{App, Arg};
use std::thread;
use std::time::Duration;

mod alert;

fn main() {
    let matches = App::new("gilfoyle-alert")
        .version("0.1.0")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("Silicon valley Gilfoyle alert")
        .arg(
            Arg::with_name("coin")
                .short("c")
                .long("coin")
                .value_name("NAME")
                .help("Sets a cryptocurrency")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("PRICE")
                .help("Set a price limit")
                .required(true)
                .index(1),
        )
        .get_matches();

    let coin = matches.value_of("coin").unwrap_or("btc");
    let price = matches.value_of("PRICE").unwrap();

    let minute = 60;

    loop {
        alert::check(String::from(coin), String::from(price));
        thread::sleep(Duration::from_secs(minute))
    }
}
