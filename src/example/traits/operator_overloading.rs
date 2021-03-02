// Copyright 2021 Ferris Project Authors. License user Apache License.

// Operator Overloading
// In Rust, many of the operators can be overloaded via traits.
// A list of the traits, such as Add, that overload operators can be found in core::ops.

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of the type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

pub fn operator_overloading() {
    println!("Foo + Bar = {:?}", Foo + Bar);
}
