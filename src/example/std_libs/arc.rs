// Copyright 2021 Ferris Project Authors. License user Apache License.

// Arc
// When share ownership between threads is need, Arc (Atomic Reference Counted) can be used.
// This struct, via the `Clone` implementation can create a reference pointer for the location of a
// value in the memory heap while increasing the reference counter. As it shares ownership between
//  threads, when the last reference pointer to a value is out of scope, the variable is droped.

use std::thread;
use std::sync::Arc;

pub fn arc_usage() {
    // This variable declaration is where it's value is specified.
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to reference in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated in the Arc
            // variable pointer's location.
            println!("{:?}", apple);
        });
    }
}