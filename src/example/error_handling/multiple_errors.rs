//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Multiple Error types
#![allow(dead_code)]

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

// v1: pulling Result out of Option.
fn double_first_v1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

// v2: pulling Result out of Option, swap the Result and Option
fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

pub fn multiple_errors() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    // raw
    // println!("The first doubled is {}", double_first(numbers));
    // println!("The first doubled is {}", double_first(empty));
    // println!("The first doubled is {}", double_first(strings));

    // v1
    // println!("The first doubled is {:?}", double_first_v1(numbers));
    // println!("The first doubled is {:?}", double_first_v1(empty));
    // println!("The first doubled is {:?}", double_first_v1(strings));

    // v2
    println!("The first doubled is {:?}", double_first_v2(numbers));
    println!("The first doubled is {:?}", double_first_v2(empty));
    println!("The first doubled is {:?}", double_first_v2(strings));

}