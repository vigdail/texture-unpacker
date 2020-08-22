use serde::Deserialize;
use serde_xml_rs::from_str;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
#[serde(rename = "TextureAtlas")]
pub struct SparrowAtlas {
    pub imagePath: String,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "SubTexture", default)]
    pub sub_textures: Vec<SubTexture>,
}

impl SparrowAtlas {
    pub fn from_str(s: &str) -> Result<Self, serde_xml_rs::Error> {
        Ok(from_str::<SparrowAtlas>(s)?)
    }
}
