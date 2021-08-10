#![deny(warnings)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;


mod cli;
mod config;
mod entry;
mod errors;
mod filter;
mod fmt;
mod source;


#[tokio::main]
async fn main() {
    cli::main().await;
}
