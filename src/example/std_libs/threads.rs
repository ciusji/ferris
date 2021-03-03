// Copyright 2021 Ferris Project Authors. License user Apache License.

// Rust provides a mechanism for spawning native OS threads via the spawn function, the argument
// of this function is moving closure.

use std::thread;

#[allow(dead_code)]
const NTHREADS: u32 = 10;

pub fn thread_usage() {
    // Make a vector to build the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }))
    }

    for child in children {
        // Wait for the thread to finish. Return a result.
        let _ = child.join();
    }
}