use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Arguments {
    string: String,
}

pub fn get_args() -> String {
    let args = Arguments::parse();
    args.string
}
