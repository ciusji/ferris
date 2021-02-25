// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::primitives::literals::literals;
use example::primitives::tuples::tuple_usage;
use example::primitives::arrays::arrays_slices;


fn main() {
    // primitives
    literals();
    tuple_usage();
    arrays_slices();

}
