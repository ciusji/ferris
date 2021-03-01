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
    // A non-copy type
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzz");

        mem::drop(farewell);
    };

    apply(diary);



}