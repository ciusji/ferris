// Copyright 2021 Ferris Project Authors. License user Apache License.

// Derive
// The compiler is capable of providing basic implementations for some traits via the #[derive]
// attribute. These traits can still be manually implemented if a more complex behavior is required.

// The following is a list of derivable traits:
// - Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// - clone, to create T from &T via a copy.
// - Copy, to give a type `copy semantics` instead of `move semantics`.
// - Hash, to compute a hash from &T.
// - Default, to crate an empty instance of a data type.
// - Debug, to format a value using the {:?} formatter.

// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes.
struct Seconds(i32);

pub fn derive() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can not be printed; it doesn't implement the `Debug` trait.
    // println!("One second looks like: {:?}", _one_second);

    // Error: `Seconds` can not be compared; it doesn't implement the `PartialEq` trait.
    // let _this_is_true = _one_second == _one_second;

    let foot = Inches(12);

    println!("one foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} that one meter.", cmp);
}
