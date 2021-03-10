//! Copyright 2021 Ferris Project Authors. License user Apache License.

// ?
// Sometimes we just want the simplicity of `unwrap` without the possibility of a `panic`. Until now,
// unwrap has forced us to nest deeper and deeper when what we really wanted was to get the variable
// out. This is exactly the purpose of ?.
// Upon finding an Err, there are two valid actions to take:
// 1. panic!, which we already decided to try to avoid if possible
// 2. return, because an Err means it cannot be handled

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    // The `try!` macro
    // Before there was ?, the same functionality was achieve with the try! macro.
    // The ? operator is now recommended, but you may still find try! when looking at older code.
    // let second_number = try!(second_number_str.parse::<i32>());
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("result is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn result_question() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}