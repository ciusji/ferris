// Copyright 2021 Ferris Project Authors. License user Apache License.
#[macro_use]
extern crate clap;

mod example;

use example::ext_libs::termcolor_usage::write_green;
use crate::example::ext_libs::clap_usage::app;

fn main() {
    let _do = write_green();
    app();
}
