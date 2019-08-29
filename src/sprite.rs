use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Frame {
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Size {
	pub w: u32,
	pub h: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Sprite {
	pub filename: String,
	pub frame: Frame,
	pub rotated: bool,
	pub trimmed: bool,
	pub spriteSourceSize: Frame,
	pub sourceSize: Size,
	pub pivot: Option<Point>,
}
