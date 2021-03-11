//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Lifetime - Structs
// Annotation on lifetimes in structures are also similar to functions.

// A type `Borrowed` which houses a reference to an `i32`.
// The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

pub fn structs() {
    let x = 18i32;
    let y = 15i32;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let referrer = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", referrer);
    println!("y is *not* borrowed in {:?}", number);
}