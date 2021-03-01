// Copyright 2021 Ferris Project Authors. License user Apache License.

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

pub fn cfg_using() {
    are_you_on_linux();
}