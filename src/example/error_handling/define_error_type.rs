//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Define an error type
// Sometimes it simplifies the code to mask all of the different errors with a single type of error.
// Rust allow us to define our own error types. In general, a "good" error type:
// * Represents different errors with the same type
// * Presents nice error message to the user
// * Is easy to compare with other types
//      - Good: Err(EmptyVec)
//      - Bad: Err("please use a vector with at least one element".to_owned())
// * Can hold information about the error
// * Composes well with other errors

use std::fmt;
use std::fmt::Formatter;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)] struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("result is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn define_error_type() {
    let numbers = vec!["1", "2", "3"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "128"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}