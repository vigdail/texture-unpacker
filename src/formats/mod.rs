pub mod json;
pub mod xml;

use crate::{error::Error, formats::xml::SparrowAtlas};
use json::JsonArrayAtlas;
use serde::Deserialize;
use std::{convert::TryFrom, ffi::OsStr, fs::File, io::Read};

pub enum AtlasFormat {
    Json,
    Xml,
}

impl TryFrom<&OsStr> for AtlasFormat {
    type Error = Error;
    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        match value.to_str().unwrap() {
            "json" => Ok(AtlasFormat::Json),
            "xml" => Ok(AtlasFormat::Xml),
            _ => Err(Error::UnknownAtlasFormat(
                value.to_str().unwrap().to_owned(),
            )),
        }
    }
}

pub struct Atlas {
    pub image_path: String,
    pub size: Size,
    pub frames: Vec<Frame>,
}

pub struct Frame {
    pub name: String,
    pub position: Point,
    pub size: Size,
    pub bound: Rect,
    pub rotated: bool,
    pub trimmed: bool,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Atlas {
    pub fn from_file(path: &str) -> Result<Atlas, Error> {
        let mut buffer: String = String::new();

        let mut file = File::open(path)?;

        file.read_to_string(&mut buffer)?;

        let format = AtlasFormat::try_from(std::path::Path::new(path).extension().unwrap())?;
        match format {
            AtlasFormat::Json => Atlas::from_json(&buffer),
            AtlasFormat::Xml => Atlas::from_xml(&buffer.as_str()),
        }
    }

    fn from_json(buffer: &str) -> Result<Atlas, Error> {
        Ok(serde_json::from_str::<JsonArrayAtlas>(&buffer)?.into())
    }

    fn from_xml(buffer: &str) -> Result<Atlas, Error> {
        let xml = SparrowAtlas::from_str(buffer)?;

        let xml_frames = xml.sub_textures;

        let mut frames = Vec::<Frame>::new();
        for frame in xml_frames.iter() {
            let sprite = Frame {
                name: frame.name.clone(),
                position: Point {
                    x: frame.x,
                    y: frame.y,
                },
                size: Size {
                    w: frame.width,
                    h: frame.height,
                },
                rotated: false,
                bound: Rect {
                    x: frame.frameX.map(|x| x.abs() as u32).unwrap_or(0),
                    y: frame.frameY.map(|y| y.abs() as u32).unwrap_or(0),
                    w: frame.frameWidth.unwrap_or(frame.width),
                    h: frame.frameHeight.unwrap_or(frame.height),
                },
                trimmed: frame.frameWidth.is_some() || frame.frameHeight.is_some(),
            };
            frames.push(sprite);
        }

        let atlas = Atlas {
            image_path: xml.image_path,
            size: Size {
                w: xml.width,
                h: xml.height,
            },
            frames,
        };
        Ok(atlas)
    }
}
