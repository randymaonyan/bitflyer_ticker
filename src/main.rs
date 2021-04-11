mod structs;
use bitflyer_ticker::get_ticker;

fn main() -> anyhow::Result<()> {
    if let Err(e) = get_ticker() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
