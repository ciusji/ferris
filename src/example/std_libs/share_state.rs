// Copyright 2021 Ferris Project Authors. License user Apache License.

// Share-State Concurrency
// Message passing is a fine way of handling concurrency, but it's not the only core. Consider this
// part of the slogan from the Go language documentation again: "do not communicate by sharing memory".

// In a way, channels in any programming language are similar to single ownership, because once you
// transfer a value down a channel, you should no longer use that value. Shared memory concurrency
// is like multiple ownership: multiple threads can access the same memory location at the same time.

// Using mutexes to allow access to data from on thread at a time.
// Mutexes have a reputation for being difficult to use because you have to remember two rules:
// - You must attempt to acquire the lock before using the data.
// - When you're done with the data that the mutex guards, you must unlock the data so other threads
//   can acquire the lock.

use std::sync::{Mutex, Arc};
use std::thread;


// Using an `Arc<T>` to wrap the `Mutex<T>` to be able to share ownership across multiple threads.
pub fn share_state() {
    // Creates a new mutex in an unlocked state ready for use.
    // let m = Mutex::new(5);
    //
    // {
    //     // This call will block the current thread so it can't do any work until it's our turn
    //     // to have the lock.
    //     // The call to lock would fail if another thread holding the lock panicked.
    //     // The call to lock returns a smart pointer called `MutexGuard`, wrapped in a LockResult that
    //     // we handled with the call to unwrap.
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // // The smart pointer also has a `Drop` implementation that releases the lock automatically
    // // when a `MutexGuard` goes out of scope.
    //
    // println!("m = {:?}", m);

    // Sharing a Mutex<T> between multiple threads
    // let counter = Mutex::new(0);
    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("x = {:?}", *counter.lock().unwrap());

}