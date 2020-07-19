use structopt::StructOpt;
use std::process::Command;
use std::io::{self, Write, Read};
use std::env::{current_dir, set_current_dir};
use std::fs::File;
use colored::*;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
struct Opt {
	/// Create crate as a library
	#[structopt(long)]
	lib: bool, 

	/// Create crate as a binary
	#[structopt(long)]
	bin: bool,

	/// Name of the crate to be made
	#[structopt(required=true)]
	name: Vec<String>,

	/// Description to be added to README.md
	#[structopt(long, default_value="")]
	description: String,

	/// Use this to add a license
	#[structopt(long, default_value="")]
	license: String,
}

fn main() {
    let opt = Opt::from_args();
	let name = if &opt.name[0]=="owo"{
		opt.name.get(1)
	} else {
		opt.name.get(0)
	};
	let name = match name {
		Some(x) => x,
		None => {
			cmdout(
				Command::new("cargo-owo")
					.output()
					.expect(&"Could not run `cargo-owo`".red())
			);
			return;
		}
	};
    
    if (opt.lib as u8 + opt.bin as u8)%2==0 {
    	println!("Please specify whether or not you would like to create a library or binary project with tags --lib or --bin respectively.");
    	return;
    }

    let proj_type = if opt.lib {"--lib"} else {"--bin"};

	let cargo_new = Command::new("cargo")
		.arg("new")
		.arg(&proj_type)
		.arg(&name)
		.output()
		.expect(&"Could not run `cargo new ...`".red());
	cmdout(cargo_new);

	let mut path = current_dir()
		.expect(&"Could not get current_dir".red());
	path.push(&name);
	set_current_dir(path)
		.expect(&"Could not set_current_dir".red());

	let mut cargo_file = File::open("Cargo.toml").unwrap();
	let mut cargo_contents = String::new();
	cargo_file.read_to_string(&mut cargo_contents).unwrap();
	cargo_contents = cargo_contents.replace(
		"edition = \"2018\"", 
		"edition = \"2018\"\nlicense-file = \"LICENSE\""
	).replace(
		"edition = \"2018\"",
		"edition = \"2018\"\nreadme = \"README.md\""
	).replace(
		"edition = \"2018\"",
		&format!("edition = \"2018\"\ndescription = \"{}\"", opt.description)
	).replace(
		"edition = \"2018\"",
		&format!("edition = \"2018\"\nrepository = \"{}\"", "")
	);

	match File::create("Cargo.toml") {
		Ok(mut f) => {
			f.write_all(cargo_contents.as_bytes())
				.expect("Could not write to Cargo.toml");
			println!("{}", "Added defaults to Cargo.toml".green());
		},
		Err(e) => println!("{}", &format!("{}", e).red())
	}

	match File::create("README.md") {
		Ok(mut f) => {
			f.write_all(&format!("# {}", &name).as_bytes())
				.expect(&"Could not write to README.md".red());
			f.write_all(&format!("\n{}", &opt.description).as_bytes())
				.expect(&"Could not write to README.md".red());
			println!("{}", "Created template README.md".green());

		},
		Err(e) => println!("{}", &format!("{}", e).red())
	}

	if &opt.license != "" {
		match File::create("LICENSE") {
			Ok(mut file) => {
				match Command::new("curl")
					.arg(&format!("https://api.github.com/licenses/{}", &opt.license)).output() {
						Ok(license_data) => {
							if let Ok(ld_str) = String::from_utf8(license_data.stdout) {
								for entry in json::parse(&ld_str).unwrap().entries() {
									if entry.0 == "body" {
										if let Some(body) = entry.1.as_str() {
											match file.write_all(body.as_bytes()) {
												Ok(_) => println!("{}", "Created LICENSE".green()),
												Err(e) => println!("{}", &format!("{}", e).red()),
											}
										}
									}
								}
							}
						},
						Err(e) => println!("{}", &format!("{}", e).red()),
					};
			},
			Err(e) => println!("{}", &format!("{}", e).red()),
		}
	}

	match File::create(".gitignore") {
		Ok(mut f) => {
			f.write_all("/target\n".as_bytes())
				.expect(&"Could not write `/target\\n` to .gitignore".red());
			if opt.lib {
				f.write_all("Cargo.lock".as_bytes())
					.expect(&"Could not write `Cargo.lock` to .gitignore".red());
			}
			println!("{}", "Created .gitignore".green());

			let git_init = Command::new("git")
				.arg("init")
				.output();
			match git_init {
				Ok(o) => { 
					cmdout(o);
					match Command::new("git").arg("add").arg(".").output() {
						Ok(o) => {
							cmdout(o);
							match Command::new("git")
								.arg("commit")
								.arg(&format!("-m \"Inital commit\""))
								.output() {
								Ok(o) => {
									cmdout(o);
								},
								Err(e) => println!("{}", &format!("{}", e).red()),
							}
						},
						Err(e) => println!("{}", &format!("{}", e).red()),
					}
				},
				Err(e) => println!("{}", &format!("{}", e).red()),
			}
		},
		Err(e) => println!("{}", &format!("{}", e).red()),
	}
	
}

fn cmdout(command: std::process::Output) {
	let [out, err] = [&command.stdout, &command.stderr];
	io::stdout().write_all(out).unwrap();
	io::stderr().write_all(err).unwrap();
}