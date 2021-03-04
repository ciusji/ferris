// Copyright 2021 Ferris Project Authors. License user Apache License.

// The File struct represents a file that has been opened (it wraps a file descriptor), and gives
// read and/or write access to the underlying file.
// Since many things can go wrong when doing file I/O, all the File methods return the io::Result<T>
//  type, which is an alias for Result<T, io::Error>.

use std::fs::File;
use std::path::Path;
use std::io::Read;

fn open_file() {
    // Create a path to the desired file
    let path = Path::new("/Users/admin/Desktop/rust_programming.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    // A File owns a resource, the file descriptor and takes care of closing the file when it
    // is drop ed.
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} contains: \n{}", display, s),
    }

    // `file` goes out of scope, and the "*.txt" file gets closed
}

pub fn file_ops() {
    open_file();
}