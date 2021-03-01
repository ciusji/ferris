// Copyright 2021 Ferris Project Authors. License user Apache License.

// Functions are declared using the fn keyword. Its arguments are type annotated, just like
// variables, and, if the function returns a value, the return type must be specified after an
// arrow `->`.

#[warn(dead_code)]

// Unlike C/C++, there's no restriction on the order of function definitions
pub fn fizz_bizz() {
    // We can use this function here, and define it somewhere later
    fb_to(100);
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Functions that "dont't" return a value, actually return the unit type '()'
fn fb(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizz-buzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the result type can be omitted form the signature
fn fb_to(n: u32) {
    for n in 1..n + 1 {
        fb(n);
    }
}