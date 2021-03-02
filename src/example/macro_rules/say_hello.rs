// Copyright 2021 Ferris Project Authors. License user Apache License.

// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    }
}

pub fn macro_hello() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}