// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use crate::example::error_handling::option_unwrap_example::option_unwrap_example;
use crate::example::error_handling::unpacking_options::unpacking_options;
use crate::example::error_handling::combinator_map::combinator_map;
use crate::example::error_handling::combinator_and_then::combinator_and_then;
use crate::example::error_handling::result_map::result_map;
use crate::example::error_handling::result_alias::result_alias;
use crate::example::error_handling::result_question::result_question;
use crate::example::error_handling::multiple_errors::multiple_errors;
use crate::example::error_handling::define_error_type::define_error_type;
use crate::example::error_handling::iterating_over_result::iterating_over_result;


fn main() {
    option_unwrap_example();
    unpacking_options();
    combinator_map();
    combinator_and_then();
    result_map();
    result_alias();
    result_question();
    multiple_errors();
    define_error_type();
    iterating_over_result();
}
