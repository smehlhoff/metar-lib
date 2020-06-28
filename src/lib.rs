// https://www.ofcm.gov/publications/fmh/FMH1/fmh1_2019.pdf
// https://tgftp.nws.noaa.gov/data/observations/metar/stations/KSJC.TXT

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

#[macro_use]
extern crate lazy_static;
use chrono::prelude::*;
use regex::Regex;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Metar {
    pub raw_data: String,
    pub data: ParsedMetar,
}

impl fmt::Display for Metar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw_data)
    }
}

impl Metar {
    pub fn parse(code: &str) -> Result<Self, Box<dyn Error>> {
        let mut raw_data = Self::fetch_data(code)?;

        raw_data = Self::split_data(code, &raw_data)?;

        let data = ParsedMetar::parse_data(&raw_data)?;

        Ok(Self { raw_data, data })
    }

    fn fetch_data(code: &str) -> Result<String, Box<dyn Error>> {
        let url =
            format!("https://tgftp.nws.noaa.gov/data/observations/metar/stations/{}.TXT", code);
        let resp = reqwest::blocking::get(&url)?.text()?;

        Ok(resp)
    }

    fn split_data(code: &str, raw_data: &str) -> Result<String, Box<dyn Error>> {
        let raw_data: Vec<&str> =
            raw_data.split('\n').filter(|&x| x != "" && x.contains(code)).collect();

        Ok(raw_data[0].to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsedMetar {
    pub station: String,
    pub station_type: String,
    pub time: DateTime<Utc>,
    pub wind: Wind,
    pub visibility: String,
    pub weather: Vec<String>,
    pub clouds: Vec<String>,
    pub temp: i32,
    pub dew: i32,
    pub altimeter: i32,
    pub remarks: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Wind {
    pub direction: i32,
    pub speed: i32,
    pub gust_speed: i32,
    pub variable_speed: i32,
}

impl Wind {
    const fn new() -> Self {
        Self { direction: 0, speed: 0, gust_speed: 0, variable_speed: 0 }
    }
}

impl ParsedMetar {
    fn new() -> Self {
        Self {
            station: String::from(""),
            station_type: String::from(""),
            time: Utc::now(),
            wind: Wind::new(),
            visibility: String::from(""),
            weather: Vec::new(),
            clouds: Vec::new(),
            temp: 0,
            dew: 0,
            altimeter: 0,
            remarks: Vec::new(),
        }
    }

    pub fn parse_data(raw_data: &str) -> Result<Self, Box<dyn Error>> {
        lazy_static! {
            static ref RE: Regex = {
                let pattern = [
                    r"(?P<station>[A-Z0-9]{4})\s",
                    r"(?P<time>\d{6}Z)\s",
                    r"(?P<type>AUTO\s|COR\s)?",
                    r"(?P<wind>VRB\d{2}KT|\d{5}KT|\d{5}G\d{2}KT)\s",
                    r"(?P<variable>\d{3}V\d{3}\s)?",
                    r"(?P<vis>\d{4}|\d{3}SM|\d{2}SM|\d{1}SM|\d{1}\s\d{1}/\d{1}SM|\d{1}/\d{1}SM
                        |\d{1}\s\d{1}.\d+SM|\d{1}.\d+SM)?\s",
                    r"(?P<weather>.+\s)",
                    r"(?P<temp>\d{2}|M\d{2})/(?P<dew>\d{2}|M\d{2})\s",
                    r"(?P<alt>A\d{4}\s)?",
                    r"(?P<remarks>RMK.+|RMK)?",
                ]
                .join("");

                Regex::new(&pattern).unwrap()
            };
        }

        if let Some(data) = RE.captures(raw_data) {
            let station = data["station"].to_string();
            let time = Self::parse_time(&data["time"]);
            let wind = Self::parse_wind(&data["wind"]);
            let (weather, clouds) = Self::parse_weather(&data["weather"])?;
            let temp = Self::parse_temp(&data["temp"]);
            let dew = Self::parse_dew(&data["dew"]);

            let station_type = match data.name("type") {
                Some(_) => match data["type"].as_ref() {
                    "AUTO " => String::from("AUTO"),
                    "COR " => String::from("COR"),
                    _ => String::from(""),
                },
                None => String::from(""),
            };

            let visibility = match data.name("vis") {
                Some(_) => Self::parse_vis(&data["vis"]),
                None => String::from(""),
            };

            let altimeter = match data.name("alt") {
                Some(_) => Self::parse_alt(&data["alt"]),
                None => 0,
            };

            let remarks = match data.name("remarks") {
                Some(_) => Self::parse_remarks(&data["remarks"]),
                None => Vec::new(),
            };

            Ok(Self {
                station,
                station_type,
                time,
                wind,
                visibility,
                weather,
                clouds,
                temp,
                dew,
                altimeter,
                remarks,
            })
        } else {
            Ok(Self::new())
        }
    }

    fn parse_time(raw_time: &str) -> DateTime<Utc> {
        let raw_time: Vec<&str> = raw_time.split("").filter(|&x| x != "" && x != "Z").collect();
        let utc = Utc::now();
        let day = raw_time[0..2].join("").parse::<u32>().unwrap();
        let hour = raw_time[2..4].join("").parse::<u32>().unwrap();
        let min = raw_time[4..6].join("").parse::<u32>().unwrap();

        Utc.ymd(utc.year(), utc.month(), day).and_hms(hour, min, 0)
    }

    fn parse_wind(raw_wind: &str) -> Wind {
        let raw_wind: Vec<&str> =
            raw_wind.split("").filter(|&x| x != "" && x != "K" && x != "T" && x != "G").collect();

        if raw_wind[0].contains('V') {
            Wind {
                direction: 0,
                speed: 0,
                gust_speed: 0,
                variable_speed: raw_wind[3..5].join("").parse::<i32>().unwrap(),
            }
        } else {
            let direction = raw_wind[0..3].join("").parse::<i32>().unwrap();
            let speed = raw_wind[3..5].join("").parse::<i32>().unwrap();
            let gust_speed =
                if raw_wind.len() > 5 { raw_wind[5..].join("").parse::<i32>().unwrap() } else { 0 };

            Wind { direction, speed, gust_speed, variable_speed: 0 }
        }
    }

    // TODO: parse variable direction
    // fn parse_variable() {}

    fn parse_vis(raw_vis: &str) -> String {
        let mut raw_vis: Vec<&str> =
            raw_vis.split("").filter(|&x| x != "" && x != " " && x != "S" && x != "M").collect();

        if raw_vis.len() >= 4 && (raw_vis[2] == "/" || raw_vis[2] == ".") {
            raw_vis.insert(1, " ");
            raw_vis.join("")
        } else {
            raw_vis.join("")
        }
    }

    fn parse_temp(raw_temp: &str) -> i32 {
        let raw_temp: Vec<&str> = raw_temp.split("").filter(|&x| x != "").collect();

        if raw_temp[0] == "M" {
            -raw_temp[1..].join("").parse::<i32>().unwrap()
        } else {
            raw_temp[0..2].join("").parse::<i32>().unwrap()
        }
    }

    fn parse_dew(raw_dew: &str) -> i32 {
        let raw_dew: Vec<&str> = raw_dew.split("").filter(|&x| x != "").collect();

        if raw_dew[0] == "M" {
            -raw_dew[1..].join("").parse::<i32>().unwrap()
        } else {
            raw_dew[0..2].join("").parse::<i32>().unwrap()
        }
    }

    fn parse_alt(raw_alt: &str) -> i32 {
        let raw_alt: Vec<&str> = raw_alt.split("").filter(|&x| x != " " && x != "A").collect();

        raw_alt[0..].join("").parse::<i32>().unwrap()
    }

    fn parse_weather(raw_weather: &str) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
        let mut raw_weather: Vec<&str> = raw_weather.split(' ').filter(|&x| x != "").collect();
        let mut clouds: Vec<String> = Vec::new();

        lazy_static! {
            static ref RE: Regex = {
                let pattern = [
                    r"(?P<clouds>",
                    r"FEW\d{3}CB",
                    r"|FEW\d{3}TCU",
                    r"|FEW\d{3}",
                    r"|SCT\d{3}CB",
                    r"|SCT\d{3}TCU",
                    r"|SCT\d{3}",
                    r"|BKN\d{3}CB",
                    r"|BKN\d{3}TCU",
                    r"|BKN\d{3}",
                    r"|OVC\d{3}CB",
                    r"|OVC\d{3}TCU",
                    r"|OVC\d{3}",
                    r"|CAVOK",
                    r"|CLR",
                    r"|SKC)",
                ]
                .join("");

                Regex::new(&pattern).unwrap()
            };
        }

        for val in &raw_weather {
            for cap in RE.captures_iter(val) {
                if let Some(val) = cap.name("clouds") {
                    clouds.push(val.as_str().to_string())
                }
            }
        }

        let index = raw_weather.iter().position(|&x| x == clouds[0]).unwrap();
        let weather: Vec<String> = raw_weather.drain(0..index).map(str::to_string).collect();

        Ok((weather, clouds))
    }

    fn parse_remarks(raw_remarks: &str) -> Vec<String> {
        raw_remarks.split(' ').map(str::to_string).filter(|x| x != "RMK").collect()
    }
}
