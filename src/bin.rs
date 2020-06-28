use metar_cli::*;

fn main() {
    let metar = "KSFO";

    println!("{:#?}", Metar::parse(metar));
}
