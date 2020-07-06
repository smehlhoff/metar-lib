use metar_lib::*;

fn main() {
    let metar = Metar::parse("KSFO").unwrap();

    println!("The temperature at {} is {} Celsius.", metar.data.station, metar.data.temp);
}
