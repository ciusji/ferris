//! Copyright 2021 Ferris Project Authors. License user Apache License.

// alias for Result
// How about when we want to reuse a specific `Result` type man times? Recall that Rust allows us to
// create aliases. Conveniently, we can define one for the specific `Result` in question.

use std::num::ParseIntError;

// Define a generic alias for a `Result` with the error `ParseIntError`
type AliasResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allow us to save some space.
fn print(result: AliasResult<i32>) {
    match result {
        Ok(n) => println!("result is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn result_alias() {
    print(multiply("10", "2"));
    print(multiply("tt", "2"));
}