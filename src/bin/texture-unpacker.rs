use structopt::StructOpt;
use texture_unpacker::SpriteSheet;

#[derive(StructOpt)]
#[structopt(author)]
struct Config {
    #[structopt(short = "o", default_value = ".", help = "set output dir")]
    dir: String,
    #[structopt(short, long, default_value = "json", help = "format of atlas")]
    format: String,
    #[structopt(help = "path to atlas file")]
    path: String,
}

fn main() {
    let config: Config = Config::from_args();

    let output_dir = config.dir;
    let atlas_type = config.format;

    let input = config.path;

    let image_name = format!("{}.png", input);
    let atlas_name = format!("{}.{}", input, atlas_type);
    let mut atlas = match SpriteSheet::load(image_name.as_str(), atlas_name.as_str()) {
        Ok(a) => a,
        Err(e) => {
            println!("File: {} skipped (Reason: {})", image_name, e);
            return;
        }
    };
    match atlas.unpack(output_dir.as_str()) {
        Ok(_) => println!("Complete"),
        Err(_) => println!("Error: can't unpack spritesheet"),
    };
}
