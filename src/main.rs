use calculator::*;

fn run() -> anyhow::Result<()> {
    let input_str: String = cli::get_args();
    let input_str: &str = input_str.as_str();
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{:?}", e);
    }
}
