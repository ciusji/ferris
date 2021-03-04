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
use example::std_libs::threads::thread_usage;
use example::std_libs::mr::mr_task;
use example::std_libs::demos::demo;
use example::std_libs::thread_spawn::thread_spawn;
use example::std_libs::move_spawn::move_thread_spawn;
use example::std_libs::message_passing::message_passing;
use example::std_libs::share_state::share_state;
use example::std_libs::channels::channels;
use example::std_libs::path::path_info;

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
    thread_usage();
    mr_task();
    demo();
    thread_spawn();
    move_thread_spawn();
    channels();
    message_passing();
    share_state();
    path_info();
}
