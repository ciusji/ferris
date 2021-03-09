// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::error_handling::option_unwrap_example::option_unwrap_example;
use example::error_handling::unpacking_options::unpacking_options;
use example::error_handling::combinator_map::combinator_map;


fn main() {
    option_unwrap_example();
    unpacking_options();
    combinator_map();
}
