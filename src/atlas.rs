use crate::sprite::Sprite;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Atlas {
	pub frames: Vec<Sprite>,
}

impl Atlas {
	// @TODO return value should be an Option
	pub fn from_file(path: &str) -> Result<Atlas,()>{
		let mut buffer: String = String::new();
		let mut file = File::open(path).unwrap();
		file.read_to_string(&mut buffer).unwrap();

		let atlas: Atlas = serde_json::from_str(&buffer.as_str()).unwrap();

		Result::Ok(atlas)
	}
}
