// Copyright 2021 Ferris Project Authors. License user Apache License.

// Rc
// When multiple ownership is needed, Rc (Reference Counting) can be used. Rc keeps track of the
// number of the references which means the number of owners of the value wrapped inside an Rc.
// Reference count of an Rc increases by 1 whenever an Rc is cloned, and decreases by 1 whenever
// one cloned Rc is dropped out of the scope. When an Rc's reference count becomes zero, which means
// there are no owners remained, both the Rc and the value are all dropped.
// Cloning an Rc never performs a deep copy. Cloning creates just another pointer to the wrapped
// value, and increments the count.

use std::rc::Rc;

pub fn rc_usage() {
    let rc_examples = "Rc example".to_string();

    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }

    // Error!
    // `rc_examples` already moved into `rc-a`
    // and when `rc_a` is dropped, `rc_examples` is dropped together
    // println!("{}", rc_examples);

}