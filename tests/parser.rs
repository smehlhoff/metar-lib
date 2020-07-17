use chrono::prelude::*;
use metar_lib::*;

#[test]
fn metar_station() {
    let metar =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();

    assert_eq!(metar.station, "KSFO");
}

#[test]
fn metar_time() {
    let metar =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let utc = Utc::now();

    assert_eq!(metar.time, Utc.ymd(utc.year(), utc.month(), 16).and_hms(4, 56, 0));
}

#[test]
fn metar_station_type() {
    let metar = ParsedMetar::parse_data(
        "KSFO 160456Z AUTO 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2",
    )
    .unwrap();
    let metar2 = ParsedMetar::parse_data(
        "KSFO 160456Z COR 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2",
    )
    .unwrap();
    let metar3 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();

    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar2.station_type, "COR");
    assert_eq!(metar3.station_type, "");
}

#[test]
fn metar_wind() {
    let metar =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar2 =
        ParsedMetar::parse_data("KSFO 160456Z VRB03KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar3 =
        ParsedMetar::parse_data("KSFO 160456Z 00000KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar4 =
        ParsedMetar::parse_data("KSFO 160456Z 10SM FEW009 SCT200 15/10 A2999 RMK AO2").unwrap();

    assert_eq!(metar.wind, Wind { direction: 270, speed: 24, gust_speed: 33, variable_speed: 0 });
    assert_eq!(metar2.wind, Wind { direction: 0, speed: 0, gust_speed: 0, variable_speed: 3 });
    assert_eq!(metar3.wind, Wind { direction: 0, speed: 0, gust_speed: 0, variable_speed: 0 });
    assert_eq!(metar4.wind, Wind { direction: 0, speed: 0, gust_speed: 0, variable_speed: 0 });
}

#[test]
fn metar_wind_variation() {
    let metar = ParsedMetar::parse_data(
        "KSFO 160456Z 27024G33KT 280V300 10SM FEW009 SCT200 15/10 A2999 RMK AO2",
    )
    .unwrap();
    let metar2 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();

    assert_eq!(metar.wind_variation, "280V300");
    assert_eq!(metar2.wind_variation, "");
}

#[test]
fn metar_vis() {
    let metar =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar2 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 1SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar3 = ParsedMetar::parse_data(
        "KSFO 160456Z 27024G33KT 1 1/2SM FEW009 SCT200 15/10 A2999 RMK AO2",
    )
    .unwrap();
    let metar4 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 1/2SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar5 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 5/16SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar6 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT M1/4SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar7 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();

    assert_eq!(metar.vis, "10");
    assert_eq!(metar2.vis, "1");
    assert_eq!(metar3.vis, "1 1/2");
    assert_eq!(metar4.vis, "1/2");
    assert_eq!(metar5.vis, "5/16");
    assert_eq!(metar6.vis, "< 1/4");
    assert_eq!(metar7.vis, "");
}

#[test]
fn metar_rvr() {
    let metar = ParsedMetar::parse_data(
        "KSFO 160456Z 27024G33KT 10SM R28/4000FT FEW009 SCT200 15/10 A2999 RMK AO2",
    )
    .unwrap();
    let metar2 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();

    assert_eq!(metar.rvr, "R28/4000FT");
    assert_eq!(metar2.rvr, "");
}

#[test]
fn metar_weather() {
    let metar = ParsedMetar::parse_data(
        "KSFO 160456Z 27024G33KT 10SM BR TSRA +HZ -FG FEW009 SCT200 15/10 A2999 RMK AO2",
    )
    .unwrap();

    assert_eq!(metar.weather, vec!["BR", "TSRA", "+HZ", "-FG"]);
}

#[test]
fn metar_clouds() {
    let metar =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar2 = ParsedMetar::parse_data(
        "KSFO 160456Z 27024G33KT 10SM FEW005CB SCT050TCU OVC800 15/10 A2999 RMK AO2",
    )
    .unwrap();
    let metar3 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM VV000 15/10 A2999 RMK AO2").unwrap();
    let metar4 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM CLR 15/10 A2999 RMK AO2").unwrap();

    assert_eq!(metar.clouds, vec!["FEW009", "SCT200"]);
    assert_eq!(metar2.clouds, vec!["FEW005CB", "SCT050TCU", "OVC800"]);
    assert_eq!(metar3.clouds, vec!["VV000"]);
    assert_eq!(metar4.clouds, vec!["CLR"]);
}

#[test]
fn metar_temp_dew() {
    let metar =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar2 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 M06/M06 A2999 RMK AO2")
            .unwrap();

    assert_eq!(metar.temp, 15);
    assert_eq!(metar.dew, 10);
    assert_eq!(metar2.temp, -6);
    assert_eq!(metar2.dew, -6);
}

#[test]
fn metar_alt() {
    let metar =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar2 = ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM CLR 15/10 RMK AO2").unwrap();

    assert_eq!(metar.alt, 2999);
    assert_eq!(metar2.alt, 0);
}

#[test]
fn metar_remarks() {
    let metar =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK AO2")
            .unwrap();
    let metar2 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999 RMK")
            .unwrap();
    let metar3 =
        ParsedMetar::parse_data("KSFO 160456Z 27024G33KT 10SM FEW009 SCT200 15/10 A2999").unwrap();

    assert_eq!(metar.remarks.len(), 1);
    assert_eq!(metar2.remarks.len(), 0);
    assert_eq!(metar3.remarks.len(), 0);
}

#[test]
fn metar_full_report() {
    let metar = ParsedMetar::parse_data("KSFO 160456Z AUTO 27024G33KT 280V300 10SM R28/4000FT +FG TSRA BKN008 OVC040 00/M08 A2992 RMK AO2").unwrap();
    let utc = Utc::now();

    assert_eq!(metar.station, "KSFO");
    assert_eq!(metar.time, Utc.ymd(utc.year(), utc.month(), 16).and_hms(4, 56, 0));
    assert_eq!(metar.station_type, "AUTO");
    assert_eq!(metar.wind, Wind { direction: 270, speed: 24, gust_speed: 33, variable_speed: 0 });
    assert_eq!(metar.wind_variation, "280V300");
    assert_eq!(metar.vis, "10");
    assert_eq!(metar.rvr, "R28/4000FT");
    assert_eq!(metar.weather, vec!["+FG", "TSRA"]);
    assert_eq!(metar.clouds, vec!["BKN008", "OVC040"]);
    assert_eq!(metar.temp, 0);
    assert_eq!(metar.dew, -8);
    assert_eq!(metar.alt, 2992);
    assert_eq!(metar.remarks.len(), 1);
}
