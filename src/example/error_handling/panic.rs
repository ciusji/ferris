// Copyright 2021 Ferris Project Authors. License user Apache License.

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAaaa!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}


pub fn simple_panic() {
    drink("water");
    drink("lemonade");
}