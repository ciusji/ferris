//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Bounds
// When working with generics, the type parameters ofter must use traits as bounds to stipulate
// what functionality a type implements. For example, the following example uses the trait Display
// to print and so it requires T to be bound by Display; that is, T must implement Display.

// Define a function printer that make a generic type T which must implement that Display.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

// Bounding restricts the generic to types that conform to be bounds. That is:
struct S<T: Dispaly>(T);

fn main() {
    // Error! Vec<T> does not implement Display. This specialization will fail.
    let _s = S(vec![1]);
}