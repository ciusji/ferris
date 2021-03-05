//! Copyright 2021 Ferris Project Authors. License user Apache License.

use std::env::args;
use std::path::PathBuf;

use structopt::StructOpt;

struct Cli {
    pattern: String,
    // Aside: `PathBuf` is like a `String` but for file system paths that works cross-platform.
    path: std::path::PathBuf,
}

/// A small `grep` clone.
///
/// That is a tool that we give a string and a path and it'll print only the lines that contains
/// the given string. Let;s call it grrs.
///
/// # Examples
///
/// ```
/// grrs foo text.txt
/// foo: 10
/// ```
pub fn grrs() {
    // Getting the arguments
    let pattern = args().nth(1).expect("no pattern given");
    let path = args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: PathBuf::from(path),
    };




}