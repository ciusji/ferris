// Copyright 2021 Ferris Project Authors. License user Apache License.

// Integers 1, floats 1.2, characters 'a', strings "abc", booleans true and the unit type()
// can by expressed using literal.

// Underscores can be inserted in numeric literals.

pub fn literals() {

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    println!("1 - 2 = {}", 1i32 - 2);

    // Boolean logic
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    // Underscore to improve readability
    println!("{}", 1_000_000u32);

}