extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate github_rs;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


mod cli;
mod config;
mod entry;
mod errors;
mod fmt;
mod source;


fn main() {
    cli::main();
}
