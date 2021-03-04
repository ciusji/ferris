// Copyright 2021 Ferris Project Authors. License user Apache License.

// Creating multiple produces by cloning the transmitter.
// `mpsc` was an acronym for multiple producer, single consumer.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn multiple_producer_single_consumer() {
    let (tx, rx) = mpsc::channel();

    // Cloning the transmitter
    let tx_x = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from(", "),
            String::from("I"),
            String::from("come"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx_x.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // println!("{:?}", rx.recv().unwrap());

    for receiver in rx {
        println!("Got: {}", receiver);
    }
    // Output like this:
    /*
        Got: hi (??? why)
        Got: hi
        Got: ,
        Got: I
        Got: come
        Got: from
        Got: the
        Got: thread
     */

}