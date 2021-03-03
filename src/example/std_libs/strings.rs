// Copyright 2021 Ferris Project Authors. License user Apache License.

// There are two types of strings in Rust: String and &str.
// A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8
// sequence . String is heap allocated, growable and not null terminated.
// &Str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view
// into a String, just like &[T] is a view into Vec<T>.

// use std::str;

pub fn strings() {
    // A reference to a string allocated in read only memory.
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

}