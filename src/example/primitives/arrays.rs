// Copyright 2021 Ferris Project Authors. License user Apache License.

// Arrays & Slices
// An array is a collection of objects of the same type T, stored in contiguous memory.
// Slices are similar to arrays, but their length is not known at compile time.

use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn arrays_slices() {
    // Fixed-size array (type signature is superfluous)
    // let xs = [1, 2, 3, 4, 5];
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of the elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slice can point to a section of an array
    // They are of the form [starting_index..ending_index]
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes compile error (error: this operation will panic at runtime)
    println!("{}", xs[5]);

}