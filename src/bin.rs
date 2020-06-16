use metar_cli::*;

fn main() {
    let metar = Metar::parse("metars/KSFO.TXT");

    println!("{:#?}", metar);
}
