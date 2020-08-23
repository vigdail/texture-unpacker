use crate::{
    error::Error,
    formats::{Atlas, Rect},
};

use image::DynamicImage;

pub struct SpriteSheet {
    pub sprite: DynamicImage,
    pub sheet: Atlas,
}

impl SpriteSheet {
    pub fn load(sprite_path: &str, atlas_path: &str) -> Result<SpriteSheet, Error> {
        let image = image::open(sprite_path)?;

        let sheet = Atlas::from_file(atlas_path)?;

        Ok(SpriteSheet {
            sprite: image,
            sheet,
        })
    }

    pub fn unpack(&mut self, path: &str) -> Result<(), ()> {
        for (i, image) in self.sheet.frames.iter().enumerate() {
            let image = image.clone();
            let position = image.position;
            let size = image.size;

            let mut sprite: DynamicImage;
            if image.rotated {
                sprite = self.sprite.crop(position.y, position.x, size.h, size.w);
            } else {
                sprite = self.sprite.crop(position.x, position.y, size.w, size.h);
            };

            if image.trimmed {
                sprite = SpriteSheet::build(&sprite, image.bound);
            }

            let s = &format!("{}/{}.png", path, &image.name);
            let path = std::path::Path::new(s);
            let dir = path.parent().unwrap();
            if !dir.exists() {
                std::fs::create_dir_all(dir).unwrap();
            }
            let percent = (i + 1) as f32 / self.sheet.frames.len() as f32 * 100.0;
            match sprite.save(path) {
                Ok(_) => println!("{}% saved: {}", percent as u32, image.name),
                Err(e) => println!("{}% skipped: {}: {}", percent as u32, image.name, e),
            }
        }

        Ok(())
    }

    fn build(image: &DynamicImage, bound: Rect) -> DynamicImage {
        let mut bottom = DynamicImage::new_rgba8(bound.w, bound.h);

        image::imageops::overlay(&mut bottom, image, bound.x, bound.y);

        bottom
    }
}
