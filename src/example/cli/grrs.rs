//! Copyright 2021 Ferris Project Authors. License user Apache License.

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
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
    // // Getting the arguments
    // let pattern = args().nth(1).expect("no pattern given");
    // let path = args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern,
    //     path: PathBuf::from(path),
    // };
    //
    // println!("{:?}", args.pattern);

    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}