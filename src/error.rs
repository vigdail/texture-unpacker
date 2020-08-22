use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),
    #[fail(display = "Unknown atlas format: {}", _0)]
    UnknownAtlasFormat(String),
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Serde(err)
    }
}
