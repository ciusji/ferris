// Copyright 2021 Ferris Project Authors. License user Apache License.

// Closure - As input parameters

// A function which takes a closure as an argument and calls it.
// <F> denoted that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothings.
    F: FnOnce() {
    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {
    f(3)
}

pub fn closure_args() {
    use std::mem;

    let greeting = "hello";



}