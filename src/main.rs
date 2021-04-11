mod structs;
use bitflyer_ticker::stream_ticker;

fn main() -> anyhow::Result<()> {
    if let Err(e) = stream_ticker() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
