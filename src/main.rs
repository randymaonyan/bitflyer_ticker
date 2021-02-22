mod structs;
use structs::Root;

use std::sync::mpsc;
use std::thread;

use anyhow::*;
use chrono::*;
use tungstenite::{connect, Message as Msg};
use url::Url;

const URL: &str = "wss://ws.lightstream.bitflyer.com/json-rpc";
const METHOD: &str =
    r#"{"jsonrpc":"2.0","method":"subscribe","params":{"channel":"lightning_ticker_BTC_JPY"}}"#;

fn main() -> anyhow::Result<()> {
    let (mut socket, response) =connect(Url::parse(URL)
        .unwrap())
        .context(format!("connection error: {}", URL))?;

    println!("connected");
    println!("code: {}", response.status());

    for (ref header, value) in response.headers() {
        println!("* {}: {:?}", header, value);
    }

    ctrlc::set_handler(|| {
        println!("Closing");
        std::process::exit(0);
    }).expect("quit error");
    
    socket.write_message(Msg::Text(METHOD.into())).context(format!("socket write error: {}", METHOD))?;

    let (tx, rx) = mpsc::channel();
    
    let handle = thread::spawn(move || loop {
        let msg = socket.read_message().unwrap();
        let msg = msg.to_text().unwrap();
        // let res: Root = serde_json::from_str(msg).expect("json parse error");
        match serde_json::from_str(msg) {
            Ok(res) => tx.send(res).unwrap(),
            Err(_) => {
                // Error("EOF while parsing a value", line: 1, column: 0)
            }
        }
    });

    for res in rx {
        show_ticker(&res);
    }

    handle.join().unwrap();
    Ok(())
}

fn show_ticker(tick: &Root) {

    let to_date = |s: &String| -> String {
        let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%f%Z").unwrap();
        dt.format("%Y/%m/%d %H:%M:%S").to_string()
    };

    println!(
        "{} {} {} {}",
        tick.params.message.product_code,
        to_date(&tick.params.message.timestamp),
        tick.params.message.volume_by_product,
        tick.params.message.ltp
    )
}
