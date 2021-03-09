//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Option & unwrap
// An enum called Option<T> in the std library is used when absence is a possibility. It manifests
// itself as one of two "options":
// - Some(T): An element of type T was found
// - None: No element was found

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

// All gifts are handled implicitly using `unwrap`
fn give_royal(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaaaaa!!!!");
    }
    println!("I love {}s!!!", inside);
}

pub fn option_unwrap_example() {
    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_royal(bird);
    give_commoner(nothing);
    // give_royal(snake);
}