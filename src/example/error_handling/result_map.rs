//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Result
// `Result` is a richer version of the `Option` type that describes possible error instead of possible
// absence.
// That is, Result<T, E> could have one of two outcomes:
// - Ok(T): An element T was found
// - Err(E): An error was found with element E
// By convention, the expected outcome is Ok while the unexpected outcome is Err.
#![allow(dead_code)]
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

// `Option`'s map, `and_then`, and many other combinators are also implemented for `Result`.
fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("result is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn result_map() {
    // This still presents a reasonable answer
    let twenty = multiply("10", "2");
    print(twenty);

    let thirty = multiply_v2("10", "3");
    print(thirty);

    // Error: invalid digit found in string.
    let tt = multiply("t", "2");
    print(tt);
}