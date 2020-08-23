use crate::formats::{Atlas, Frame, Point, Rect, Size};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonArrayAtlas {
    meta: Meta,
    frames: Vec<JsonArrayFrame>,
}

#[derive(Deserialize)]
pub struct Meta {
    image: String,
    size: Size,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JsonArrayFrame {
    filename: String,
    frame: Rect,
    rotated: bool,
    trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: Rect,
    #[serde(rename = "sourceSize")]
    source_size: Size,
    pivot: Option<Point>,
}

impl From<JsonArrayFrame> for Frame {
    fn from(frame: JsonArrayFrame) -> Self {
        Frame {
            name: frame.filename.clone(),
            position: Point {
                x: frame.frame.x,
                y: frame.frame.y,
            },
            size: Size {
                w: frame.frame.w,
                h: frame.frame.h,
            },
            bound: Rect {
                x: frame.sprite_source_size.x,
                y: frame.sprite_source_size.y,
                w: frame.source_size.w,
                h: frame.source_size.h,
            },
            trimmed: frame.trimmed,
            rotated: frame.rotated,
        }
    }
}

impl From<JsonArrayAtlas> for Atlas {
    fn from(atlas: JsonArrayAtlas) -> Atlas {
        let frames = atlas.frames.into_iter().map(|frame| frame.into()).collect();
        Atlas {
            image_path: atlas.meta.image,
            size: atlas.meta.size,
            frames,
        }
    }
}
