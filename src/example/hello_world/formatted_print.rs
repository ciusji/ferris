// Copyright 2021 Ferris Project Authors. License user Apache License.

// Printing is handled by a series of macros defined in std::fmt some of which include:
// - format!: write formatted text to string
// - print!: same as format! but the text is printed to the console (io::stdout)
// - println!: same as print! but a newline is appended
// - eprint!: same as format! but the text is printed to the standard error (io::stderr)
// - eprintln!: same as eprint! but a newline is appended

pub fn print() {
    // In general, the `{}` will be automatically replaced with any argument.
    // Without a suffix, 31 becomes an i32.
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

    // Special formatting can be specified after `:`
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width.
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes.
    println!("{number:>0width$}", number=1, width=6);

}