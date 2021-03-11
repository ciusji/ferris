// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use crate::example::scoping_rules::structs::structs;
use crate::example::scoping_rules::traits::traits;
use crate::example::scoping_rules::bounds::bounds;

fn main() {
    structs();
    traits();
    bounds();
}
