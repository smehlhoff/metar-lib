use std::fmt;
use std::num;

#[derive(Debug)]
pub enum Error {
    Invalid(String),
    NotFound(String),
    ParseInt(num::ParseIntError),
    Regex(regex::Error),
    Reqwest(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Invalid(ref err) | Self::NotFound(ref err) => write!(f, "{}", err),
            Self::ParseInt(ref err) => write!(f, "{}", err),
            Self::Regex(ref err) => write!(f, "{}", err),
            Self::Reqwest(ref err) => write!(f, "{}", err),
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Self::Regex(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}
