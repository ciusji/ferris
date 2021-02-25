// Copyright 2021 Ferris Project Authors. License user Apache License.

// Rust provides type safety static typing. Variable bindings can be type annotated when declared.
// However, in most cases, the compiler will be able to infer the type of the variable from the context,
// heavily reducing the annotation burden.

pub fn bindings() {
    // Variable bindings are immutable by default, but this can be overridden using the mut modifier.
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    // _immutable_binding += 1;
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

}