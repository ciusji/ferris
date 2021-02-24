// Copyright 2021 Ferris Project Authors. License user Apache License.

// Import via `use`
use std::fmt;

// Define a structure for which `Display` will be implemented.
// This is a tuple struct name `Structure` that contains an `i32`.
struct Structure(i32);

impl fmt::Display for Structure {
    // This trait requires `fmt` with this extract signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn display() {
    let l = Structure(1);
    println!("{}", l);
}