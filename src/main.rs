// Copyright 2021 Ferris Project Authors. License user Apache License.

mod example;

// use example::functions::fizz_bizz::fizz_bizz;
use example::functions::method::methods;
use example::functions::closures::closures;
use example::functions::closure_capturing::capturing;
use example::functions::closure_pars::closure_args;
use example::functions::closure_input::closure_input;
use example::functions::higher_order_functions::higher_order_functions;


fn main() {
    // functions
    // fizz_bizz();
    methods();
    closures();
    capturing();
    closure_args();
    closure_input();
    higher_order_functions();
}
