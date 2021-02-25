// Copyright 2021 Ferris Project Authors. License user Apache License.

// Enum
// The enum keyword allows the creation of a type which may be one of a few different variants.

enum WebEvent {
    // An `enum` may either be `unit-like`
    PageLoad,
    PageUnload,
    // like tuple structs
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={} y={}", x, y);
        },
    }
}

pub fn enums() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted =  WebEvent::Paste("my text".to_owned());
    let click =   WebEvent::Click { x: 20, y: 80 };
    let load =    WebEvent::PageLoad;
    let unload =  WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}