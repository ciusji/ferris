//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Where clauses
// A bound can also be expressed using a `where` clause immediately before the opening {, rather
// that at the type's first mention. Additionally, `where` clause can apply bounds to arbitrary
// types, rather that just to tpe parameters.

use std::fmt::Debug;

trait PrintOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as T: Debug or use another method of indirect
// approach this requires a `where` clause.
impl<T> PrintOption for T where
    Option<T>: Debug {
    // We want `Option<T>: Debug` as our bound because that is what's being printed.
    // Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

pub fn where_clause() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
