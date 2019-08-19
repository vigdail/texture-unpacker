use crate::atlas::Atlas;
use crate::sprite::{Frame, Size, Sprite};

use image::DynamicImage;

pub struct SpriteSheet {
	pub sprite: DynamicImage,
	pub sheet: Atlas,
}

impl SpriteSheet {
	pub fn load(sprite_path: &str, json_path: &str) -> SpriteSheet {
		let image = image::open(sprite_path).unwrap();
		let sheet = Atlas::from_file(json_path);

		SpriteSheet {
			sprite: image,
			sheet,
		}
	}

	pub fn unpack(&mut self) -> Result<(), ()> {
		for image in self.sheet.frames.iter() {
			let image: Sprite = image.clone();
			let frame: Frame = image.frame;

			let mut sprite: DynamicImage;
			if image.rotated {
				sprite = self.sprite.crop(frame.y, frame.x, frame.h, frame.w);
			} else {
				sprite = self.sprite.crop(frame.x, frame.y, frame.w, frame.h);
			};

			if image.trimmed {
				sprite = SpriteSheet::build(&sprite, image.sourceSize, image.spriteSourceSize);
			}

			let s = &format!("res/files/{}.png", &image.filename);
			let path = std::path::Path::new(s);
			let dir = path.parent().unwrap();
			if !dir.exists() {
				std::fs::create_dir_all(dir).unwrap();
			}
			println!("{:?}", path);
			sprite.save(path).unwrap();
		}

		Ok(())
	}

	fn build(image: &DynamicImage, size: Size, source_size: Frame) -> DynamicImage {
		let mut bottom = DynamicImage::new_rgba8(size.w, size.h);

		image::imageops::overlay(&mut bottom, image, source_size.x, source_size.y);

		bottom
	}
}
