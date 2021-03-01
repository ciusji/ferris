// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::modules::module::module;
use example::modules::super_self::super_self;

fn main() {
    // modules
    module();
    super_self();
}
