mod model;
mod parse;
mod processing;
mod storage;
mod display;

use std::process::exit;

use model::Error;
use parse::parse;
use processing::process_command;

fn _main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let command = parse(&args)?;
    process_command(command)
}

fn main() {
    let result = _main();
    if let Err(e) = result {
        eprintln!("{e}");
        exit(1);
    }
}
