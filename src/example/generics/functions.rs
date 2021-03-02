// Copyright 2021 Ferris Project Authors. License user Apache License.

// The same set of rules can be applied to functions: a type T becomes generic when preceded by <T>.

struct A;
struct S(A);
struct SGen<T>(T);

// The following functions all take ownership of the variable passed into them and
// immediately go out of the scope, freeing the variable.

// This has no `<T>` so this is not a generic function.
fn reg_fn(_s: S) {}

// It has been explicitly given the type parameter `A`, but because `A` has not been
// specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn gen_spec_t(_s: SGen<A>) {}

// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// This function is generic over `T`.
fn generic<T>(_s: SGen<T>) {}


pub fn functions() {
    //
}

