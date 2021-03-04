// Copyright 2021 Ferris Project Authors. License user Apache License.

// Channel
// Rust provide asynchronous channels for communication between threads. Channels allow a
// unidirectional flow of information between two end-points: `Sender` and the `Receiver`.

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

#[allow(dead_code)]
static NTHREADS: i32 = 3;

#[warn(dead_code)]
pub fn channels() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>` where T is the type of
    // the message to be transferred.
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let mut children = Vec::new();

    for i in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over thread_tx
            // Each thread queues a message in the channel
            thread_tx.send(i).unwrap();

            // Sending is a non-blocking operation, the thread will continue immediately after
            // sending its message
            println!("thread {} finished", i);
        });

        children.push(child);
    }

    // Here, all the message are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv`
        ids.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);

}