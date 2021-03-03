// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::std_libs::box_stack_heap::box_stack_heap;
use example::std_libs::vectors::vectors;
use example::std_libs::strings::strings;
use example::std_libs::options::options;
use example::std_libs::results::result;
use example::std_libs::hashmap::hash_map;
use example::std_libs::hashset::hash_set;
use example::std_libs::rc::rc_usage;
use example::std_libs::arc::arc_usage;

fn main() {
    box_stack_heap();
    vectors();
    strings();
    options();
    result();
    hash_map();
    hash_set();
    rc_usage();
    arc_usage();
}
