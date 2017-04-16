extern crate env_logger;
extern crate clap;

// extern crate graph_flow;

use std::env;

use clap::{App, Arg};

const DESCRIPTION: &'static str = "graph-flow";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// use graph_flow::core;

fn main() {
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .arg(
            Arg::with_name("verbose")
            .help("Verbose mode")
            .short("v")
            .long("verbose")
            .multiple(true)
        )
        .get_matches();

    match matches.occurrences_of("verbose") {
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        3 => env::set_var("RUST_LOG", "debug"),
        _ => {}
    }

    env_logger::init().unwrap();
}
