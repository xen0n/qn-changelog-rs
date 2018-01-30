extern crate docopt;
extern crate github_rs;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


mod cli;
mod config;
mod entry;
mod fmt;
mod source;


fn main() {
    cli::main();
}
