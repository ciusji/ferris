//! Copyright 2021 Ferris Project Authors. License user Apache License.

// lifetime - trait
// Annotation of lifetime in trait methods basically are similar to functions. Note that impl may
// have annotation of lifetime too.

// A struct with annotation of lifetime
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetime to impl
impl<'a> Default for Borrowed<'a>  {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

pub fn traits() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}