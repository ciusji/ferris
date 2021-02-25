// Copyright 2021 Ferris Project Authors. License user Apache License.

// Types
// Rust provides several mechanisms to change or define the type of primitive and use defined types.
// The `type` statement can be used to give a new name to an existing type. Types must have `UpperCamelCase`
// names, or the compiler will raise a warning.

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

pub fn types() {
    // Casting
    // Rust provides no implicit type conversion between primitive types. But, explicit type
    // conversion can be performed using the `as` keyword.
    let decimal = 65.4321_f32;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits are kept, while the rest towards the most
    // significant bit get truncated.
    println!("1000 as a u8 is: {}", 1000 as u8);

    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // `size_of_val`
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));

}