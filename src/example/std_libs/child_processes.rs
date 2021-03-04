// Copyright 2021 Ferris Project Authors. License user Apache License.

// Child Processes
// The process::Output struct represents the output of a finished child process, and the
// process::Command struct is a process builder.

use std::process::{Command, Stdio};
use std::io::{Write, Read};

fn process_builder_demo() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was: \n{}", s);
    } else {
        let s = String ::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was: \n{}", s);
    }
}

// The std::Child struct represents a running child process, and exposes the stdin, stdout and stderr
// handles for interaction with the underlying process via pipes.
fn child_process_pipe() {
    let pangram = String::from("the quick brown fox jumped over the lazy dog\n");
    // Spawn `wc` command
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    //Write a string to the `stdin` of `wc`
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance must have one, we can
    // directly `unwrap` it.
    match process.stdin.unwrap().write_all(pangram.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed, and the pipe closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => println!("wc responded with:\n{}", s),
    }
}

pub fn child_processes() {
    process_builder_demo();
    child_process_pipe();

    // Wait
    // If you'd like to wait for a process::Child to finish, you must call Child::wait, which will
    // return a process::ExitStatus.
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main.");
}
