//! Copyright 2021 Ferris Project Authors. License user Apache License.

/// termcolor: a simple cross platform library for writing colored text to a terminal.
/// referrer to https://crates.io/crates/termcolor.

use std::io::{self, Write};
use termcolor::{Color, StandardStream, ColorChoice, WriteColor, ColorSpec};

// Example: using standard stream
pub fn write_green() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "green text!")
}