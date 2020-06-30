use metar_cli::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = &args[1];

    if code.len() != 4 {
        panic!("Please enter a valid METAR code.")
    };

    println!("{:#?}", Metar::parse(code));
}
