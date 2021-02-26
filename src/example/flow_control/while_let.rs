// Copyright 2021 Ferris Project Authors. License user Apache License.

// while let

pub fn while_let() {
    // Make optional of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructure `optional` into
    // `Some(i)`, evaluate the block (``{}). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }

}