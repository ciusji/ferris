// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use crate::example::generics::traits::traits;
use crate::example::generics::multiple_bounds::multiple_bounds;
use crate::example::generics::where_clauses::where_clause;

fn main() {
    traits();
    multiple_bounds();
    where_clause();
}
