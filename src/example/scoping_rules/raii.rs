// Copyright 2021 Ferris Project Authors. License user Apache License.

// RAII, Resource Acquisition Is Initialization.

fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed.
}

fn main() {
    // Allocate _box2 = Box::new(5i32);

    // A nested scope
    {
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed.
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed.
}

// We can double check for memory errors using valgrind.
// $ rustc raii.rs && valgrind ./raii