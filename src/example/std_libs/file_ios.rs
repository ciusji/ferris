// Copyright 2021 Ferris Project Authors. License user Apache License.

// The File struct represents a file that has been opened (it wraps a file descriptor), and gives
// read and/or write access to the underlying file.
// Since many things can go wrong when doing file I/O, all the File methods return the io::Result<T>
//  type, which is an alias for Result<T, io::Error>.

use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn open_file() {
    // Create a path to the desired file（panic! No such file or directory）
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

fn file_create() {
    let path = Path::new("/Users/admin/Desktop/lorem_ipsum.txt");
    let display = path.display();

    // Open a file in write-only mode, return `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREN_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn file_ops() {
    open_file();
    file_create();
}