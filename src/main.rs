// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use crate::example::error_handling::option_unwrap_example::option_unwrap_example;
use crate::example::error_handling::unpacking_options::unpacking_options;
use crate::example::error_handling::combinator_map::combinator_map;
use crate::example::error_handling::combinator_and_then::combinator_and_then;
use crate::example::error_handling::result_map::result_map;


fn main() {
    option_unwrap_example();
    unpacking_options();
    combinator_map();
    combinator_and_then();
    result_map();
}
