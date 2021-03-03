// Copyright 2021 Ferris Project Authors. License user Apache License.

// Option
// Sometimes it's desirable to catch the failure of some parts of a program instead of calling
// panic!; this can be accomplished using the Option enum.

// The Option<T> enum has two variants:
// None, to indicate failure or lack of value, and
// Some(value), a tuple struct that wraps a value with Type T.

fn check_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match check_division(dividend, divisor) {
        None => println!("{} / {} failed", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient);
        },
    }
}

pub fn options() {
    try_division(3, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated.
    let _none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`.
    // Panics if the self value equals [`None`]
    // println!("{:?} unwraps to {:?}", none, none.unwrap());

}