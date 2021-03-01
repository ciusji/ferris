// Copyright 2021 Ferris Project Authors. License user Apache License.

// Modules
// By default, the items in a module have private visibility, but this can be overridden with the
// pub modifier. Only the public items of a module can be accessed from outside the module scope.

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        content: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                content: contents,
            }
        }
    }
}

pub fn module() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The open box contents: {}", open_box.contents);

    // However, structs with private fields can be created using public constructors.
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // error: aborting due to previous error
    println!("The closed box contains: {}", _closed_box.contents);

}