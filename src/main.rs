// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::hello_world::hello::hello;
use example::hello_world::formatted_print::print;
use example::hello_world::debug::debug;
use example::hello_world::display::display;
use example::hello_world::formatting::formatting;


fn main() {
    // examples
    // hello world
    hello();
    print();
    debug();
    display();
    formatting();

}
