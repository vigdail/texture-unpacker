use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Xml(#[cause] serde_xml_rs::Error),
    #[fail(display = "Unknown atlas format: {}", _0)]
    UnknownAtlasFormat(String),
    #[fail(display = "{}", _0)]
    Json(#[cause] serde_json::error::Error),
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
        Error::Json(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Error {
        Error::Xml(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Error {
        Error::Image(err)
    }
}
