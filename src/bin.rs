use metar_cli::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = &args[1];

    println!("{:#?}", Metar::parse(code));
}
