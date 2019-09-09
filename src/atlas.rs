use crate::sprite::{Frame, Size, Sprite};
use crate::xml_atlas:: XmlAtlas;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Atlas {
	pub frames: Vec<Sprite>,
}

impl Atlas {
	pub fn from_file(path: &str) -> Result<Atlas, String> {
		let mut buffer: String = String::new();

		let mut file = match File::open(path) {
			Ok(f) => f,
			Err(e) => return Err(format!("{:?}", e)),
		};

		match file.read_to_string(&mut buffer) {
			Err(e) => return Err(format!("{:?}", e)),
			_ => {}
		};

		let ext = match std::path::Path::new(path).extension() {
			Some(e) => e.to_str(),
			None => return Err(String::from("Unknown format")),
		};

		match ext {
			Some("json") => Atlas::from_json(&buffer.as_str()),
			Some("xml") => Atlas::from_xml(&buffer.as_str()),
			Some(format) => Err(format!("Unknown format: {}", format)),
			None => Err(String::from("Unknown format")),
		}
	}

	fn from_json(buffer: &str) -> Result<Atlas, String> {
		match serde_json::from_str(&buffer) {
			Ok(atlas) => Ok(atlas),
			Err(e) => Err(format!("Can not parse json: {}", e)),
		}
	}

	fn from_xml(buffer: &str) -> Result<Atlas, String> {
		let xml = XmlAtlas::from_buffer(buffer);

		let xml_frames = xml.TextureAtlas;

		let mut frames = Vec::<Sprite>::new();
		for frame in xml_frames.iter() {
			let sprite = Sprite {
				filename: frame.name.clone(),
				frame: Frame {
					x: frame.frameX as u32,
					y: frame.frameY as u32,
					w: frame.frameWidth as u32,
					h: frame.frameHeight as u32,
				},
				rotated: false,
				trimmed: false,
				spriteSourceSize: Frame {
					x: frame.frameX as u32,
					y: frame.frameY as u32,
					w: frame.frameWidth as u32,
					h: frame.frameHeight as u32,
				},
				sourceSize: Size {
					w: frame.frameWidth as u32,
					h: frame.frameHeight as u32,
				},
				pivot: None,
			};
			frames.push(sprite);
		}

		let atlas = Atlas { frames };
		Ok(atlas)
	}
}
