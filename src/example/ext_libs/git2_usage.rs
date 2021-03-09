//! Copyright 2021 Ferris Project Authors. License user Apache License.

use std::env;
use git2::{RemoteCallbacks, Cred};


pub fn push() {
    let home = env::var("HOME").unwrap();
    println!("{}", home);

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username, _allowed_types| {
        Cred::ssh_key(
            username.unwrap(),
            None,
            std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

}