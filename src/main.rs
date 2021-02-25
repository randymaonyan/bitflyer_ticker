mod structs;
use structs::Root;

use std::sync::mpsc;
use std::thread;

use anyhow::*;
use chrono::*;
use serde_json::json;
use tungstenite::{connect, Message as Msg};
use url::Url;

const ENDPOINT: &str = "wss://ws.lightstream.bitflyer.com/json-rpc";

fn main() -> anyhow::Result<()> {
    // エンドポイントに接続
    let (mut socket, response) = connect(Url::parse(ENDPOINT).unwrap())
        .context(format!("connection error: {}", ENDPOINT))?;

    println!("connected");
    println!("code: {}", response.status());

    for (ref header, value) in response.headers() {
        println!("* {}: {:?}", header, value);
    }

    // Ctrl+C押下時の処理
    ctrlc::set_handler(|| {
        println!("Closing");
        std::process::exit(0);
    })
    .expect("quit error");

    let req = json!({
        "jsonrpc": "2.0",
        "method": "subscribe",
        "params": {
            "channel": "lightning_ticker_BTC_JPY"
        }
    });

    // サブスクライブ
    socket.write_message(Msg::Text(req.to_string())).unwrap();

    let (tx, rx) = mpsc::channel();

    // Tickerの送信スレッド
    thread::spawn(move || loop {
        let msg = socket.read_message().unwrap();
        let msg = msg.to_text().unwrap();

        // たまにinvalidなレスポンスが返却されるため、Errは無視
        match serde_json::from_str(msg) {
            Ok(res) => tx.send(res).unwrap(),
            Err(_) => {
                // Error("EOF while parsing a value", line: 1, column: 0)
            }
        }
    });

    // Tickerの受信
    for res in rx {
        show_ticker(&res);
    }

    Ok(())
}

fn show_ticker(tick: &Root) {
    let to_date = |s: &String| -> String {
        let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%f%Z").unwrap();
        dt.format("%Y/%m/%d %H:%M:%S").to_string()
    };

    println!(
        "{} {} {}",
        to_date(&tick.params.message.timestamp),
        tick.params.message.volume_by_product,
        tick.params.message.ltp
    )
}
