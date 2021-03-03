// Copyright 2021 Ferris Project Authors. License user Apache License.

// Vectors are re-sizable arrays. Like slices, their size is not known at compile time, but they
// can grown or shrink at any time. A vector is represented using 3 parameters:
// - pointer to the data
// - length
// - capacity

pub fn vectors() {
    // Iterators can be allocated into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The vec! macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    xs.push(4);
    xs.push(5);
    println!("Vector: {:?}", xs);
    println!("Current xs length: {}", xs.len());

    // Indexing element
    println!("Second element in xs is: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    let last_element = xs.pop();
    println!("Last element in xs is: {:?}", last_element);

    // `Vector` can be easily iterated over
    println!("Contents of xs: ");
    for x in xs.iter() {
        println!("\t> {}", x);
    }

    // `Vector` can also be iterated over while the iteration count is enumerated in a
    // separated variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }

    println!("Updated vector: {:?}", xs);

}