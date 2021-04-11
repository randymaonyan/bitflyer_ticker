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

/// bitflyer APIを実行する関数
pub fn get_ticker() -> anyhow::Result<()> {
    // APIエンドポイントに接続
    let (mut socket, response) = connect(Url::parse(ENDPOINT).unwrap())
        .context(format!("connection error: {}", ENDPOINT))?;

    println!("connected");
    println!("code: {}", response.status());
    for (ref header, value) in response.headers() {
        println!("* {}: {:?}", header, value);
    }

    // Ctrl+C押下時の処理の定義
    ctrlc::set_handler(|| {
        println!("Closing connection");
        std::process::exit(0);
    })
    .expect("quit error");

    // APIリクエスト用JSONパラメータを作成
    let req = json!({
        "jsonrpc": "2.0",
        "method": "subscribe",
        "params": {
            "channel": "lightning_ticker_BTC_JPY"
        }
    });

    // APIリクエストを送信
    socket.write_message(Msg::Text(req.to_string())).unwrap();

    // APIレスポンスの受信、送信用チャネルの作成
    let (tx, rx) = mpsc::channel();

    // レスポンスの受信、チャネルの送信スレッド
    thread::spawn(move || loop {
        // APIレスポンスの読取り
        let msg = socket.read_message().unwrap();
        let msg = msg.to_text().unwrap();

        // たまにinvalidなレスポンスが返却されるため、Errは無視する
        match serde_json::from_str(msg) {
            Ok(res) => tx.send(res).unwrap(),
            Err(_) => {
                // Error("EOF while parsing a value", line: 1, column: 0)
            }
        }
    });

    // チャネルの受信
    for res in rx {
        show_ticker(&res);
    }

    Ok(())
}

/// APIのJSONレスポンスの整形、表示を行う関数
fn show_ticker(tick: &Root) {
    // 日付を整形するクロージャ
    // 2021-01-01T00:00:00.1234Zみたいな日付がAPIから返ってくるので整形
    let to_date = |s: &String| -> String {
        let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%f%Z").unwrap();
        // 2021/01/01 00:00:00の形式でReturnする
        dt.format("%Y/%m/%d %H:%M:%S").to_string()
    };

    println!(
        "{} {} {}",
        to_date(&tick.params.message.timestamp),
        tick.params.message.volume_by_product,
        tick.params.message.ltp
    )
}
