// Copyright 2021 Ferris Project Authors. License user Apache License.

// dyn

// The Rust compiler needs to know how much space every function's return type requires. This
// means all your functions have to return a concrete type. Unlike other languages, if you have a
// trait like Animal, you can not write a function that returns Animal, because its different
// implementations will need different amounts of memory.

struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which
// one at compile time.
// A box is just a reference to some memory in the heap.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

// the trait bound `dyn Animal: std::marker::Sized` is not satisfied [E0277] `dyn Animal` does
// not have a constant size known at compile-time
/*
fn random_animal_2(random_number: f64) -> Animal {
    if random_number < 0.5 {
        Sheep {}
    } else {
        Cow {}
    }
}
*/

pub fn returning_trait() {
    let random_number = 0.234;
    let animal = random_animal(random_number);

    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
