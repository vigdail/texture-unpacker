use failure::Fail;
use std::{io, num::ParseIntError};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Xml(#[cause] ParseIntError),
    #[fail(display = "Unknown atlas format: {}", _0)]
    UnknownAtlasFormat(String),
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::error::Error),
    #[fail(display = "{}", _0)]
    Image(#[cause] image::ImageError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::Serde(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::Xml(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Error {
        Error::Image(err)
    }
}
