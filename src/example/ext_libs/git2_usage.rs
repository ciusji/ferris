//! Copyright 2021 Ferris Project Authors. License user Apache License.

use std::env;

pub fn push() {
    let home = env::var("HOME").unwrap();
    println!("{}", home);
}