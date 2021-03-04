// Copyright 2021 Ferris Project Authors. License user Apache License.

use std::thread;

// Attempting to use a vector created by the main thread in another thread
pub fn move_thread_spawn() {
    let v = vec![1i32, 2, 3];

    // error[E0373]: closure may outlive the current function, but it borrows `v`,
    // which is owned by the current function
    // so?
    // By adding the `move` keyword before the closure, we force the closure to take ownership of
    // the values it's using rather that allowing Rust to infer that it should borrow the values.
    let handle = thread::spawn(move || {
        // The closure uses v, so it will capture v and make it part of the closure's environment.
        // Rust can't tell how long the spawned thread will run, so it doesn't know if the reference
        // v will always be valid.
        println!("Here is a vector: {:?}", &v);
    });

    handle.join().unwrap();
}