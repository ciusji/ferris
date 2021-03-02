// Copyright 2021 Ferris Project Authors. License user Apache License.

// The Iterator trait is used to implement iterators over collections such as arrays.
// The trait requires only a method to be defined fo for the next element, which may be manually
// defined in an impl block or automatically defined (as in array and ranges).
// As a point of convenience for common situations, the for construct turns some collections into
// iterators using the .into_iter() method.

struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    // * When the `Iterator` is finished, `None` is returned.
    // * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator` will never return
        // `None`, and `Some` is always returned.
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}


pub fn iterator_fib() {
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next()); // Some(2)
    println!("> {:?}", sequence.next()); // None
    println!("> {:?}", sequence.next()); // None

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterator through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(4)` method reduces an `Iterator` to its first `4` terms.
    // The `skip(4)` method shortens an `Iterator` by dropping its first `4` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterator the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

}