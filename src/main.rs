use structopt::StructOpt;
use std::process::Command;
use std::io::{self, Write};
use std::env::{current_dir, set_current_dir};
use std::fs::File;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
struct Opt {
	#[structopt(long)]
	lib: bool, 

	#[structopt(long)]
	bin: bool,

	#[structopt(long, required=true)]
	name: String,

	#[structopt(long, default_value="")]
	description: String,

	#[structopt(name="ignore", required=false, default_value="")]
	ignore: String,
}

fn main() {
    let opt = Opt::from_args();
    
    if (opt.lib as u8 + opt.bin as u8)%2==0 {
    	println!("Please specify whether or not you would like to create a library or binary project with tags --lib or --bin respectively.");
    	return;
    }

    let proj_type = if opt.lib {"--lib"} else {"--bin"};

	let cargo_new = Command::new("cargo")
		.arg("new")
		.arg(&proj_type)
		.arg(&opt.name)
		.output()
		.expect("Could not run `cargo new ...`");
	cmdout(cargo_new);

	let mut path = current_dir()
		.expect("Could not get current_dir");
	path.push(&opt.name);
	set_current_dir(path)
		.expect("Could not set_current_dir");

	match File::create("README.md") {
		Ok(mut f) => {
			f.write_all(&format!("# {}", &opt.name).as_bytes())
				.expect("Could not write to README.md");
			f.write_all(&format!("\n{}", &opt.description).as_bytes())
				.expect("Could not write to README.md");
			println!("Created template README.md");

		},
		Err(e) => println!("{}", e)
	}

	match File::create(".gitignore") {
		Ok(mut f) => {
			f.write_all("/target\n".as_bytes())
				.expect("Could not write `/target\\n` to .gitignore");
			if opt.lib {
				f.write_all("Cargo.lock".as_bytes())
					.expect("Could not write `Cargo.lock` to .gitignore");
			}
			println!("Created .gitignore");

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
								Err(e) => println!("{}", e),
							}
						},
						Err(e) => println!("{}", e),
					}
				},
				Err(e) => println!("{}", e),
			}
		},
		Err(e) => println!("{}", e),
	}
	
}

fn cmdout(command: std::process::Output) {
	let  [out, err] = [&command.stdout, &command.stderr];
	io::stdout().write_all(out).unwrap();
	io::stderr().write_all(err).unwrap();
} 