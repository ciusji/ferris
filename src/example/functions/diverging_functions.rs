// Copyright 2021 Ferris Project Authors. License user Apache License.

// Diverging Function (发散函数)
// Diverging functions never return. They are marked using `!`, which is an empty type.

fn foo() -> ! {
    panic!("This call never returns.")
}

// As opposed to all the other types, this one cannot be instantiated, because the set of all possible
// values this type can have is empty. Note that, it is different form the `()` type, which has exactly
// one possible value.