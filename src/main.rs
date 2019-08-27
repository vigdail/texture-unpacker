mod atlas;
mod sprite;
mod spritesheet;

extern crate getopts;
extern crate image;
extern crate serde;
extern crate serde_json;

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

	let matches = opts.parse(&args[1..]).unwrap();
	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}

	let output = match matches.opt_str("o") {
		Some(o) => o,
		None => String::from("./"),
	};

	let input = match matches.free.get(0) {
		Some(i) => i,
		None => {
			print_usage(&program, opts);
			return;
		}
	};

	let image_name = format!("{}.png", input);
	let json_name = format!("{}.json", input);;
	let mut atlas = match SpriteSheet::load(image_name.as_str(), json_name.as_str()) {
		Some(a) => a,
		None => {
			println!("File: {} skipped", image_name);
			return;
		}
	};
	match atlas.unpack(output.as_str()) {
		Ok(_) => println!("Complete"),
		Err(_) => println!("Error: can't unpack spritesheet"),
	};
}
