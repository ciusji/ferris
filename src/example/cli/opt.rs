//! Copyright 2021 Ferris Project Authors. License user Apache License.

/// A basic example

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ferris")]
struct Opt {
    /// Active debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    verbose: u8,

    /// Set speed
    #[structopt(short, long, default_value = "42")]
    speed: f64,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`
    /// Number of cars
    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[structopt(short, long)]
    level: Vec<String>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

pub fn opt() {
    let options = Opt::from_args();
    println!("{:#?}", options);

}