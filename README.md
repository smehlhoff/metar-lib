# metar-lib

Parses latest METARs from NOAA Aviation Weather Center IAW [FCM-H1-2019](https://www.ofcm.gov/publications/fmh/FMH1/fmh1_2019.pdf).

Do not use this library for real world navigation.

## Usage

```rust
use metar_lib::*;

fn main() {
    let metar = Metar::parse("KSFO").unwrap();

    println!("The temperature at {} is {} Celsius.", metar.data.station, metar.data.temp);
}
```

## Limitations

This library parses U.S. METARs only (e.g., KSFO, KJFK).

Please open an issue if you find a METAR string that doesn't work.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://github.com/smehlhoff/metar-lib/blob/master/LICENSE)