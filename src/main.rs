// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

use example::conversion::from_into::from_into;
use example::conversion::try_from_into::try_from_into;
use example::conversion::strings::strings;

fn main() {
    // conversion
    from_into();
    try_from_into();
    strings();
}
