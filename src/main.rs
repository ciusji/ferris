// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::custom_types::structures::structures;
use example::custom_types::enums::enums;
use example::custom_types::constants::constants;


fn main() {
    // custom types
    structures();
    enums();
    constants();
}
