// Copyright 2021 Ferris Project Authors. License user Apache License.

use std::fmt;

struct Circle {
    radius: i32,
}

// Converting to string
// To convert any type to a string is as simple as implementing the ToString trait for the type.
// Rather than doing so directly, you should implement the fmt::Display trait which auto-magically
// Provides ToString and also allows printing the type as discussed in the section on print!.
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

pub fn strings() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parsing a string
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);
}