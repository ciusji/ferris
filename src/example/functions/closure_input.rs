// Copyright 2021 Ferris Project Authors. License user Apache License.

// Closures - Input Functions
// Since closures may be used as arguments, you might wonder if the same ban be said about
// functions. And indeed they can! If you declare a function that takes a closure as parameter, then
// any function that satisfies the trait bound of that closure can be passed as a parameter.

pub fn closure_input() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I', a closure!");

    call_me(closure);
    call_me(function);
}

// Define a function which takes a generic `F` argument bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the 'Fn' bound
fn function() {
    println!("I'm a function!")
}