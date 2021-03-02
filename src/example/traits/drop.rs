// Copyright 2021 Ferris Project Authors. License user Apache License.

// Drop
// The Drop trait only has one method: drop, which is called automatically when an object goes out
// of scope. The main use of the Drop trait is to free the resources that the implementor instance
// owns.

// Box, Vec, String, File and Process are some examples of types that implement the Drop trait
// to free resources. The Drop trait can also be manually implemented for any custom data type.

// The following example adds a print to console to the drop function to announce when it is called.

struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

pub fn drop_trait() {
    let _a = Droppable { name: "a" };

    // Block A
    {
        let _b = Droppable { name: "b" };

        // Block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B")
        }
        println!("Just exit block B");

        println!("Exit block A");
    }
    println!("Just exit block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);

    println!("end of the main function");

    // `_a` won't be `drop`ed again here, because it already has been manually dropped.

}