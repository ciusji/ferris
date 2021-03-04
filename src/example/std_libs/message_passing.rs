// Copyright 2021 Ferris Project Authors. License user Apache License.

// A channel in programming has two halves: a transmitter and a receiver.
// The transmitter half is the upstream location where you put rubber ducks into the river, and the
// receiver half is where the rubber duck ends up downstream. One part of your code calls methods on
// the transmitter with the data you want to send, and another part checks the receiving end for
// arriving messages. A channel is said to be closed if either the transmitter or receiver half is
// dropped.

use std::sync::mpsc;
use std::thread;

pub fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // `move` occurs because `val` has type `String`, which does not implement the `Copy` trait
        let val = String::from("hi");
        // Calling unwrap to panic in case of an error.
        // But in real application, we would handle it properly.
        tx.send(val).unwrap();

        // Error: borrow of moved value: `val`
        // The send function takes ownership of its parameter, and when the value is moved, the
        // receive takes ownership of it. This stop us from accidentally using the value again after
        // sending it.
        // println!("val is {}", val);
    });

    let message = rx.recv().unwrap();
    println!("Got: {}", message);
}