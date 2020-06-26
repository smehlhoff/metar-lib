use metar_cli::*;

fn main() {
    let code = "KSJC";

    println!("{:?}", Metar::raw(code));
    println!("{:#?}", Metar::parse(code));
}
