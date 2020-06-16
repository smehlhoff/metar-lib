use chrono::prelude::*;
use metar_cli::*;

#[test]
fn metar_ksfo() {
    let metar = ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2 PK WND 27035/0442 SLP155 T01500100").unwrap();

    assert_eq!(metar.station, "KSFO");
    assert_eq!(metar.station_type, "");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(4, 56, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 270,
            speed: 24,
            gust: true,
            gust_speed: 33,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "10");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["FEW009", "SCT200"]);
    assert_eq!(metar.temp, 15);
    assert_eq!(metar.dew, 10);
    assert_eq!(metar.altimeter, 2999);
    assert_eq!(metar.remarks.len(), 7);
}

#[test]
fn metar_ktad() {
    let metar = ParsedMetar::parse_data("KTAD 160454Z AUTO 24024G33KT 10SM CAVOK 23/03 A3011 RMK AO2 PK WND 24033/0450 SLP097 T02330033").unwrap();

    assert_eq!(metar.station, "KTAD");
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(4, 54, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 240,
            speed: 24,
            gust: true,
            gust_speed: 33,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "10");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["CAVOK"]);
    assert_eq!(metar.temp, 23);
    assert_eq!(metar.dew, 03);
    assert_eq!(metar.altimeter, 3011);
    assert_eq!(metar.remarks.len(), 7);
}

#[test]
fn metar_mgyf() {
    let metar = ParsedMetar::parse_data(
        "KGYF 160515Z AUTO 10023KT 10SM SCT019 27/23 A3008 RMK AO2 T02720230",
    )
    .unwrap();

    assert_eq!(metar.station, "KGYF");
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(5, 15, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 100,
            speed: 23,
            gust: false,
            gust_speed: 0,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "10");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["SCT019"]);
    assert_eq!(metar.temp, 27);
    assert_eq!(metar.dew, 23);
    assert_eq!(metar.altimeter, 3008);
    assert_eq!(metar.remarks.len(), 3);
}

#[test]
fn metar_kc83() {
    let metar =
        ParsedMetar::parse_data("KC83 160515Z AUTO 23022G31KT 10SM CAVOK 16/11 A2995 RMK AO2")
            .unwrap();

    assert_eq!(metar.station, "KC83");
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(5, 15, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 230,
            speed: 22,
            gust: true,
            gust_speed: 31,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "10");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["CAVOK"]);
    assert_eq!(metar.temp, 16);
    assert_eq!(metar.dew, 11);
    assert_eq!(metar.altimeter, 2995);
    assert_eq!(metar.remarks.len(), 2);
}

#[test]
fn metar_kedw() {
    let metar = ParsedMetar::parse_data(
        "KEDW 160458Z 24011KT 50SM SCT250 BKN350 24/06 A2988 RMK AO2A SLP097 T02420055",
    )
    .unwrap();

    assert_eq!(metar.station, "KEDW");
    assert_eq!(metar.station_type, "");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(4, 58, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 240,
            speed: 11,
            gust: false,
            gust_speed: 0,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "50");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["SCT250", "BKN350"]);
    assert_eq!(metar.temp, 24);
    assert_eq!(metar.dew, 06);
    assert_eq!(metar.altimeter, 2988);
    assert_eq!(metar.remarks.len(), 4);
}

#[test]
fn metar_kqb4() {
    let metar = ParsedMetar::parse_data("KQB4 160456Z AUTO 00000KT 9000 HZ FEW095 32/03 A2986 RMK AO2 SLP080 WND DATA ESTMD T03180027").unwrap();

    assert_eq!(metar.station, "KQB4");
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(4, 56, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 0,
            speed: 0,
            gust: false,
            gust_speed: 0,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "9000");
    assert_eq!(metar.weather, vec!["HZ"]);
    assert_eq!(metar.clouds, vec!["FEW095"]);
    assert_eq!(metar.temp, 32);
    assert_eq!(metar.dew, 03);
    assert_eq!(metar.altimeter, 2986);
    assert_eq!(metar.remarks.len(), 7);
}

#[test]
fn metar_kqb3() {
    let metar = ParsedMetar::parse_data("KQD3 160456Z VRB06KT 9000 HZ FEW150 38/15 A2970 RMK AO2 SLPNO WND DATA ESTMD T03780150 10351 20299 51013 PWINO $").unwrap();

    assert_eq!(metar.station, "KQD3");
    assert_eq!(metar.station_type, "");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(4, 56, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 0,
            speed: 0,
            gust: false,
            gust_speed: 0,
            variable: true,
            variable_speed: 6
        }
    );
    assert_eq!(metar.visibility, "9000");
    assert_eq!(metar.weather, vec!["HZ"]);
    assert_eq!(metar.clouds, vec!["FEW150"]);
    assert_eq!(metar.temp, 38);
    assert_eq!(metar.dew, 15);
    assert_eq!(metar.altimeter, 2970);
    assert_eq!(metar.remarks.len(), 12);
}

#[test]
fn metar_kqay() {
    let metar = ParsedMetar::parse_data(
        "KQAY 160456Z AUTO 00000KT 9999 CAVOK 40/M01 A2957 RMK AO2 SLP013 WND DATA ESTMD T03991008",
    )
    .unwrap();

    assert_eq!(metar.station, "KQAY");
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(4, 56, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 0,
            speed: 0,
            gust: false,
            gust_speed: 0,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "9999");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["CAVOK"]);
    assert_eq!(metar.temp, 40);
    assert_eq!(metar.dew, -1);
    assert_eq!(metar.altimeter, 2957);
    assert_eq!(metar.remarks.len(), 7);
}

#[test]
fn metar_k0co() {
    let metar = ParsedMetar::parse_data(
        "K0CO 160500Z AUTO 00000KT 10SM CAVOK 07/00 A3058 RMK AO2 TS PWINO",
    )
    .unwrap();

    assert_eq!(metar.station, "K0CO");
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(5, 0, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 0,
            speed: 0,
            gust: false,
            gust_speed: 0,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "10");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["CAVOK"]);
    assert_eq!(metar.temp, 07);
    assert_eq!(metar.dew, 0);
    assert_eq!(metar.altimeter, 3058);
    assert_eq!(metar.remarks.len(), 4);
}

#[test]
fn metar_kmlp() {
    let metar = ParsedMetar::parse_data(
        "KMLP 160522Z AUTO VRB03KT 2SM BR BKN004 BKN013 OVC019 03/03 A2996 RMK AO2 T00330028 $",
    )
    .unwrap();

    assert_eq!(metar.station, "KMLP");
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(5, 22, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 0,
            speed: 0,
            gust: false,
            gust_speed: 0,
            variable: true,
            variable_speed: 3
        }
    );
    assert_eq!(metar.visibility, "2");
    assert_eq!(metar.weather, vec!["BR"]);
    assert_eq!(metar.clouds, vec!["BKN004", "BKN013", "OVC019"]);
    assert_eq!(metar.temp, 3);
    assert_eq!(metar.dew, 3);
    assert_eq!(metar.altimeter, 2996);
    assert_eq!(metar.remarks.len(), 4);
}

#[test]
fn metar_kqbt() {
    let metar = ParsedMetar::parse_data("KQBT 160455Z AUTO 04014G20KT 9999 CAVOK 39/M08 A2963 RMK AO2 SLP938 WND DATA ESTMD T03851079 TS $").unwrap();

    assert_eq!(metar.station, "KQBT");
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(4, 55, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 40,
            speed: 14,
            gust: true,
            gust_speed: 20,
            variable: false,
            variable_speed: 0
        }
    );
    assert_eq!(metar.visibility, "9999");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["CAVOK"]);
    assert_eq!(metar.temp, 39);
    assert_eq!(metar.dew, -8);
    assert_eq!(metar.altimeter, 2963);
    assert_eq!(metar.remarks.len(), 9);
}

#[test]
fn metar_kqaj() {
    let metar = ParsedMetar::parse_data(
        "KQAJ 160458Z VRB04KT 9999 CAVOK 38/M01 A2960 RMK SLPNO ALSTG ESTMD WND DATA ESTMD $",
    )
    .unwrap();

    assert_eq!(metar.station, "KQAJ");
    assert_eq!(metar.station_type, "");
    assert_eq!(metar.time, Utc.ymd(2020, 06, 16).and_hms(4, 58, 0));
    assert_eq!(
        metar.wind,
        Wind {
            direction: 0,
            speed: 0,
            gust: false,
            gust_speed: 0,
            variable: true,
            variable_speed: 4
        }
    );
    assert_eq!(metar.visibility, "9999");
    assert_eq!(metar.weather.len(), 0);
    assert_eq!(metar.clouds, vec!["CAVOK"]);
    assert_eq!(metar.temp, 38);
    assert_eq!(metar.dew, -1);
    assert_eq!(metar.altimeter, 2960);
    assert_eq!(metar.remarks.len(), 8);
}
