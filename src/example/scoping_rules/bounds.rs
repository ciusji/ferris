//! Copyright 2021 Ferris Project Authors. License user Apache License.

// lifetime - bounds
// Just like generic types can be bounded, lifetime (themselves generic) use bounds are sell. The
// `:` character has a slightly different meaning here, but `+` is the same. Note how the following
// read:
// 1. `T: 'a`: All references in T must outlive lifetime 'a.
// 2. `T: Trait + 'a`: Type T must implement trait Trait and all references ni T must outlive 'a.

use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// A generic function prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

pub fn bounds() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}