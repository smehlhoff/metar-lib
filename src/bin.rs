use metar_cli::*;

fn main() {
    println!("{:#?}", Metar::parse("KSJC"));
}
