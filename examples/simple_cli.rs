use metar_lib::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let metar = &args[1];

    println!("{:#?}", Metar::parse(metar));
}
