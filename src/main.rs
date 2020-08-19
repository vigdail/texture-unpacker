mod atlas;
mod sprite;
mod spritesheet;
mod xml_atlas;

use getopts::Options;
use spritesheet::SpriteSheet;
use std::env;

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} FILE [options]", program);
	println!("{}", opts.usage(&brief));
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optopt("o", "", "set output dir", "DIR");
	opts.optflag("h", "help", "prints this help menu");
	opts.optopt(
		"t",
		"type",
		"type of atlas [json/xml], default is json",
		"TYPE",
	);

	let matches = opts.parse(&args[1..]).unwrap();
	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}

	let output = match matches.opt_str("o") {
		Some(o) => o,
		None => String::from("./"),
	};

	let atlas_type = match matches.opt_str("t") {
		Some(t) => String::from(t.trim()),
		None => String::from("json"),
	};

	let input = match matches.free.get(0) {
		Some(i) => i,
		None => {
			print_usage(&program, opts);
			return;
		}
	};

	let image_name = format!("{}.png", input);
	let atlas_name = format!("{}.{}", input, atlas_type);
	let mut atlas = match SpriteSheet::load(image_name.as_str(), atlas_name.as_str()) {
		Ok(a) => a,
		Err(e) => {
			println!("File: {} skipped (Reason: {})", image_name, e);
			return;
		}
	};
	match atlas.unpack(output.as_str()) {
		Ok(_) => println!("Complete"),
		Err(_) => println!("Error: can't unpack spritesheet"),
	};
}
