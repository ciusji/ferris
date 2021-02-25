// Copyright 2021 Ferris Project Authors. License user Apache License.

// From and Into
// The From and Into traits are inherently linked, and this is actually part of its implementation.
// If you are able to convert type A to B, then it should be easy to believe that we should be
// able to convert type B to A.

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn from_into() {
    // From
    let my_str = "hello";
    let _my_string = String::from(my_str);

    let num = Number::from(30);
    println!("my number is {:?}", num);

    // The `into` trait is simply the reciprocal of the From trait.
    let number: Number = num.into();
    println!("my number is {:?}", number);
}