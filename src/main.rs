// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::std_libs::box_stack_heap::box_stack_heap;
use example::std_libs::vectors::vectors;

fn main() {
    box_stack_heap();
    vectors();
}
