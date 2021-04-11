mod structs;
use bitflyer_ticker::stream_ticker;

fn main() {
    if let Err(e) = stream_ticker() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
