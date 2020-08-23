use crate::formats::{Atlas, Frame, Point, Rect, Size};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubTexture {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "frameX")]
    pub frame_x: Option<i32>,
    #[serde(rename = "frameY")]
    pub frame_y: Option<i32>,
    #[serde(rename = "frameWidth")]
    pub frame_width: Option<u32>,
    #[serde(rename = "frameHeight")]
    pub frame_height: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "TextureAtlas")]
pub struct SparrowAtlas {
    #[serde(rename = "imagePath")]
    pub image_path: String,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "SubTexture", default)]
    pub sub_textures: Vec<SubTexture>,
}

impl SparrowAtlas {
    pub fn from_str(s: &str) -> Result<Self, serde_xml_rs::Error> {
        serde_xml_rs::from_str::<SparrowAtlas>(s)
    }
}

impl From<SubTexture> for Frame {
    fn from(frame: SubTexture) -> Self {
        Frame {
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
                x: frame.frame_x.map(|x| x.abs() as u32).unwrap_or(0),
                y: frame.frame_y.map(|y| y.abs() as u32).unwrap_or(0),
                w: frame.frame_width.unwrap_or(frame.width),
                h: frame.frame_height.unwrap_or(frame.height),
            },
            trimmed: frame.frame_width.is_some() || frame.frame_height.is_some(),
        }
    }
}

impl From<SparrowAtlas> for Atlas {
    fn from(atlas: SparrowAtlas) -> Self {
        let frames = atlas
            .sub_textures
            .into_iter()
            .map(|frame| frame.into())
            .collect();

        Atlas {
            image_path: atlas.image_path,
            size: Size {
                w: atlas.width,
                h: atlas.height,
            },
            frames,
        }
    }
}
