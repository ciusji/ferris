// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::traits::derive::derive;
use example::traits::operator_overloading::operator_overloading;
use example::traits::returning_trait::returning_trait;
use example::traits::trait_generic::generic;
use example::traits::drop::drop_trait;
use example::traits::iterators::iterator_fib;

fn main() {
    // Trait
    generic();
    derive();
    returning_trait();
    operator_overloading();
    drop_trait();
    iterator_fib();
}
