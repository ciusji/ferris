// Copyright 2021 Ferris Project Authors. License user Apache License.

// Use
// The use declaration can be used so manual scoping isn't needed.

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

pub fn match_use() {
    // Explicitly `use` each name so they are available without manual scoping.
    use crate::Status::{ Poor, Rich };
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldiers fight!"),
    }
}