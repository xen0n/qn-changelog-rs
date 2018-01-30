extern crate docopt;
extern crate github_rs;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


mod cli;


fn main() {
    cli::main();
}
