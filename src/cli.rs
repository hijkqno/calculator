use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Arguments {
    #[arg(allow_hyphen_values = true)]
    string: String,
}

pub fn get_args() -> String {
    let args = Arguments::parse();
    args.string
}
