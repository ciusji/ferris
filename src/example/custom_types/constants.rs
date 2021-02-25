// Copyright 2021 Ferris Project Authors. License user Apache License.

// Constants

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

pub fn constants() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);

    println!("16 is bigger than {:?}: {:?}", THRESHOLD, is_big(n));

}