// Copyright 2021 Ferris Project Authors. License user Apache License.

// Constants
// Rust has two different types of constants which can be declared in any scope including global.
// Both require explict type annotation.
// - const: An unchangeable value (the common case).
// - static: A possible mutable variable with 'static' lifetime. The static lifetime is inferred and
//      does not have to be specified. Accessing or modifying a mutable static variable is unsafe.
// See also: 'static lifetime, The const/static RFC

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