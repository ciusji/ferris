// Copyright 2021 Ferris Project Authors. License user Apache License.

// Expressions
// Blocks are expression too, so they can be used as values in assignments. The last expression in
// the block will be assigned to the place expression such as a local variable. However, if the last
// expression of the block ends with a semicolon, the return value will be ().

pub fn expression() {
    let x = 5u32;

    let y = {
        let x_squared = x ^ 2;
        let x_cube = x_squared * x;
        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

}