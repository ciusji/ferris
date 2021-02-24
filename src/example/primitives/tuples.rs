// Copyright 2021 Ferris Project Authors. License user Apache License.

// A tuple is a collection of values of different types.

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

pub fn tuple_usage() {
    let long_tuple = ( 1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a',
        true);

    // Using tuple index to extract value
    println!("long tuple index=0 is {}", long_tuple.0);
    println!("long tuple index=10 is {}", long_tuple.10);

    // Tuples are printable
    let tuple_of_tuples = ((1u8, 2u32), (4u64, -1i32, true));
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But too long can not be printed (limited size 12)
    // let long_tuples = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let long_tuples = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("long tuples numbers: {:?}", &long_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    // Reverse pair
    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, comma is required
    println!("one element tuple: {:?}", (5u32, ));
    println!("just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("a={:?} b={:?} c={:?} d={:?}", a, b, c, d);

}