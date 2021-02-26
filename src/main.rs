// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::flow_control::infinite_loop::infinite_loop;
use example::flow_control::for_loops::for_loops;
use example::flow_control::match_flow::matches;


fn main() {
    // flow of control
    infinite_loop();
    for_loops();
    matches();
}
