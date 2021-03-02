// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::traits::trait_generic::generic;
use example::traits::derive::derive;

fn main() {
    // Trait
    generic();
    derive();
}

