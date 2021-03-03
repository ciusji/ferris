// Copyright 2021 Ferris Project Authors. License user Apache License.

// HashMap
// Where vectors store values by an integer index, HashMaps store values by key. HashMap keys can be
// booleans, integers, strings, or any other type that implements the Eq and Hash traits. More on
// this in the next section.
// Like vectors, HashMaps are growable, but HashMaps can also shrink themselves when they have excess
// space. You can crate a HashMap with a certain starting capacity using HashMap::with_capacity(unit),
// or use HashMap::new() to get a HashMap with a default initial capacity (recommended).

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

pub fn hash_map() {
    let mut contacts = HashMap::new();

    // Insert data
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    println!("{}", contacts.get(&"Daniel").unwrap());

    contacts.remove(&"Daniel");

    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number))
    }

}