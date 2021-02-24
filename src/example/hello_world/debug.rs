// Copyright 2021 Ferris Project Authors. License user Apache License.

// All types which want to use `std:fmt` formatting `traits` require an implementation to be
// printable.

#[derive(Debug)]
struct Structure(i32);

pub fn debug() {
    // Printing with `{:?}` is similar to with `{}`
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's");

    // `Structure` is printable
    println!("Now {:?} will print!", Structure(3));

}