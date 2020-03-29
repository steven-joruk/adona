use std::{convert, fmt, io, num};

#[derive(Debug)]
pub enum Error {
    AssetRegexDidNotMatch,
    ResponseMissingDownloadCount,
    ResponseMissingVersion,
    ResponseMissingReleaseDate,
    ResponseMissingDownloadAddress,
    Chrono(chrono::format::ParseError),
    Io(io::Error),
    Num(num::ParseIntError),
    Json(json::Error),
    Regex(regex::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Zip(zip::result::ZipError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

impl convert::From<chrono::format::ParseError> for Error {
    fn from(error: chrono::format::ParseError) -> Self {
        Error::Chrono(error)
    }
}

impl convert::From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error::Num(error)
    }
}

impl convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl convert::From<json::Error> for Error {
    fn from(error: json::Error) -> Self {
        Error::Json(error)
    }
}

impl convert::From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::Regex(error)
    }
}

impl convert::From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl convert::From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error)
    }
}

impl convert::From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Error::Zip(error)
    }
}
