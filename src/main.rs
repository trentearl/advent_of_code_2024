use clap::Parser;
use levels::{run_level_01a, run_level_01b};
use util::{read_input_string, Level, Result};

mod levels;
mod util;

#[derive(Parser)]
pub struct Args {
    #[arg(value_enum)]
    level: Level,

    input: String,
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let input_path = args.input;
    let str = read_input_string(&input_path)?;

    match args.level {
        Level::Level01a => run_level_01a(&str),
        Level::Level01b => run_level_01b(&str),
    }
}
