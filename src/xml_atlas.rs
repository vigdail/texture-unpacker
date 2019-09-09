use std::io::Read;
use std::path::Path;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct SubTexture {
	pub name: String,
	pub x: Option<i32>,
	pub y: Option<i32>,
	pub width: Option<i32>,
	pub height: Option<i32>,
	pub frameX: i32,
	pub frameY: i32,
	pub frameWidth: i32,
	pub frameHeight: i32,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct XmlAtlas {
	pub TextureAtlas: Vec<SubTexture>,
}

impl XmlAtlas {
	pub fn from_buffer(buffer: &str) -> XmlAtlas {
		let root: minidom::Element = buffer.parse().unwrap();
		let mut frames = Vec::<SubTexture>::new();
		for child in root.children() {
			if child.name() == "SubTexture" {
				let t = SubTexture {
					name: String::from(child.attr("name").unwrap()),
					x: child.attr("x").map(|x| x.parse::<i32>().unwrap()),
					y: child.attr("y").map(|y| y.parse::<i32>().unwrap()),
					width: child.attr("width").map(|x| x.parse::<i32>().unwrap()),
					height: child.attr("height").map(|x| x.parse::<i32>().unwrap()),
					frameX: child
						.attr("frameX")
						.map(|x| x.parse::<i32>())
						.unwrap()
						.unwrap(),
					frameY: child
						.attr("frameY")
						.map(|x| x.parse::<i32>())
						.unwrap()
						.unwrap(),
					frameWidth: child
						.attr("frameWidth")
						.map(|x| x.parse::<i32>())
						.unwrap()
						.unwrap(),
					frameHeight: child
						.attr("frameHeight")
						.map(|x| x.parse::<i32>())
						.unwrap()
						.unwrap(),
				};
				frames.push(t);
			}
		}

		XmlAtlas {
			TextureAtlas: frames,
		}
	}
}
