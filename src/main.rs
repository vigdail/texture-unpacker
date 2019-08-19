mod atlas;
mod sprite;
mod spritesheet;

extern crate image;
extern crate serde;
extern crate serde_json;

use spritesheet::SpriteSheet;

fn main() {
	let mut atlas = SpriteSheet::load("res/atlas.png", "res/atlas.json");
	atlas.unpack().unwrap();

	println!("Hello, world!");
}
