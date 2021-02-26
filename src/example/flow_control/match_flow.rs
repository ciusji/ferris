// Copyright 2021 Ferris Project Authors. License user Apache License.

// Switch
// Rust provides pattern matching via the match keyword, which can be used like a C switch.
// The first matching arm is evaluated and all possible values must be covered.

pub fn matches() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        // Match a single value
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    // A match guard can be added to filter the arm.
    let pair = (2, -2);

    println!("tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}