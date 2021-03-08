//! Copyright 2021 Ferris Project Authors. License user Apache License.
use clap::{App, Arg, ArgMatches};
use std::io;
use termcolor::{StandardStream, ColorChoice};

pub fn app() {
    let _cli_flags: ArgMatches = App::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminla")
        .arg(
            Arg::with_name("1")
                .long("1 ...")
                .help("Thia is 1 ...")
        )
        .arg(
            Arg::with_name("2")
                .long(" 2...")
                .help("That is 2 ...")
        )
        .get_matches();

    let stdio = io::stdin();
    let _input = stdio.lock();
    let _output = StandardStream::stdout(ColorChoice::AlwaysAnsi);

    // use std::fs;

    // the `?` operator can only be used in a function that returns `Result` or `Option`
    // (or another type that implements `Try`)
    // E0277 Help: the trait `Try` is not implemented for `()`
    // Note: required by `from_error
    // fs::metadata("/some/file/path.txt")?;

}