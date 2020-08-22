use std::num::ParseIntError;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct SubTexture {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub frameX: Option<i32>,
    pub frameY: Option<i32>,
    pub frameWidth: Option<u32>,
    pub frameHeight: Option<u32>,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct SparrowAtlas {
    pub TextureAtlas: Vec<SubTexture>,
}

impl SparrowAtlas {
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let root: minidom::Element = s.parse().unwrap();
        let mut frames = Vec::<SubTexture>::new();
        for child in root.children() {
            if child.name() == "SubTexture" {
                let sub_texture = SubTexture {
                    name: String::from(child.attr("name").unwrap()),
                    x: child.attr("x").map(|x| x.parse::<u32>()).unwrap()?,
                    y: child.attr("y").map(|y| y.parse::<u32>()).unwrap()?,
                    width: child.attr("width").map(|x| x.parse::<u32>()).unwrap()?,
                    height: child.attr("height").map(|x| x.parse::<u32>()).unwrap()?,
                    frameX: child.attr("frameX").map(|x| x.parse::<i32>().unwrap()),
                    frameY: child.attr("frameY").map(|x| x.parse::<i32>().unwrap()),
                    frameWidth: child.attr("frameWidth").map(|x| x.parse::<u32>().unwrap()),
                    frameHeight: child.attr("frameHeight").map(|x| x.parse::<u32>().unwrap()),
                };
                frames.push(sub_texture);
            }
        }

        Ok(SparrowAtlas {
            TextureAtlas: frames,
        })
    }
}
