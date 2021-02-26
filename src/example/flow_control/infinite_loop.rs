// Copyright 2021 Ferris Project Authors. License user Apache License.

// Rust provides a loop keyword to include an infinite loop.

pub fn infinite_loop() {
    let mut count = 0u32;

    println!("let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            // Skip the rest of the iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough.");

            // Exit the loop
            break;
        }
    }

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    assert_eq!(result, 20);
}