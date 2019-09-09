use std::{convert, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(json::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    GitHubResponse,
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vaniman Error...")
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
