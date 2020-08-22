use crate::sprite::{Frame, Size, Sprite};
use crate::{error::Error, xml_atlas::XmlAtlas};

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{convert::TryFrom, ffi::OsStr, io::Read};

pub enum AtlasFormat {
    JsonArray,
    Xml,
}

impl TryFrom<&OsStr> for AtlasFormat {
    type Error = Error;
    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        match value.to_str().unwrap() {
            "json" => Ok(AtlasFormat::JsonArray),
            "xml" => Ok(AtlasFormat::Xml),
            _ => Err(Error::UnknownAtlasFormat(
                value.to_str().unwrap().to_owned(),
            )),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Atlas {
    pub frames: Vec<Sprite>,
}

impl Atlas {
    pub fn from_file(path: &str) -> Result<Atlas, Error> {
        let mut buffer: String = String::new();

        let mut file = File::open(path)?;

        file.read_to_string(&mut buffer)?;

        let format = AtlasFormat::try_from(std::path::Path::new(path).extension().unwrap())?;
        match format {
            AtlasFormat::JsonArray => Atlas::from_json(&buffer),
            AtlasFormat::Xml => Atlas::from_xml(&buffer.as_str()),
        }
    }

    fn from_json(buffer: &str) -> Result<Atlas, Error> {
        Ok(serde_json::from_str(&buffer)?)
    }

    fn from_xml(buffer: &str) -> Result<Atlas, Error> {
        let xml = XmlAtlas::from_str(buffer)?;

        let xml_frames = xml.TextureAtlas;

        let mut frames = Vec::<Sprite>::new();
        for frame in xml_frames.iter() {
            let sprite = Sprite {
                filename: frame.name.clone(),
                frame: Frame {
                    x: frame.x,
                    y: frame.y,
                    w: frame.width,
                    h: frame.height,
                },
                rotated: false,
                spriteSourceSize: Frame {
                    x: frame.frameX.map(|x| x.abs() as u32).unwrap_or(0),
                    y: frame.frameY.map(|y| y.abs() as u32).unwrap_or(0),
                    w: frame.frameWidth.unwrap_or(frame.width),
                    h: frame.frameHeight.unwrap_or(frame.height),
                },
                sourceSize: Size {
                    w: frame.frameWidth.unwrap_or(frame.width),
                    h: frame.frameHeight.unwrap_or(frame.height),
                },
                trimmed: frame.frameWidth.is_some() || frame.frameHeight.is_some(),
                pivot: None,
            };
            frames.push(sprite);
        }

        let atlas = Atlas { frames };
        Ok(atlas)
    }
}
