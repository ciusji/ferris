// Copyright 2021 Ferris Project Authors. License user Apache License.

// Create a library.
//$ rustc --crate-type=lib ?.rs
//$ ls lib*
//library.rlib

pub fn public_function() {
    println!("called x's `public_function()`");
}

fn private_function() {
    println!("called x's `private_function()`");
}

pub fn indirect_access() {
    println!("called x's `indirect_access()`, that\n> ");
    private_function();
}