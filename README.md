# cargo-owo [![Crates.io](https://img.shields.io/crates/v/cargo-owo)](https://crates.io/crates/cargo-owo)
cargo owo is a terribly-named program i wrote to automate how i start up a new project :) needs lots of work, but seems to be in a semi-usable state!

# install
to install, use the following command:
```
cargo install cargo-owo
```
(the directory it installs to must be in your PATH)

# usage
```
cargo owo <crate name> <--lib / --bin> [--description "<description>"] [--license <license name>]
```

e.g., `cargo owo mylibrary --lib --description "my library crate" --license mit` will create a library crate called "mylibrary" with a README.md with the description and a LICENSE file with given license.

run `cargo owo --help` to get the auto-generated help :)

# to-do
- make it so --description and --license updates the Cargo.toml
- make the code less of a mess lol