use crate::sprite::Sprite;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Atlas {
	pub frames: Vec<Sprite>,
}

impl Atlas {
	pub fn from_file(path: &str) -> Result<Atlas, std::io::Error> {
		let mut buffer: String = String::new();

		let mut file = match File::open(path) {
			Ok(f) => f,
			Err(e) => return Err(e),
		};

		match file.read_to_string(&mut buffer) {
			Err(e) => return Err(e),
			_ => {}
		};

		let atlas: Atlas = serde_json::from_str(&buffer.as_str()).unwrap();

		Ok(atlas)
	}
}
