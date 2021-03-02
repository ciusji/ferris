// Copyright 2021 Ferris Project Authors. License user Apache License.

// Generics is the topic of generalizing types and functionalities to broader cases. This is
// extremely useful for reducing code duplication in many ways, but can call for rather involving
// syntax. Namely, being generic requires taking great cate to specify over which types a generic
// type is actually considered valid. The simplest and most common use of generics is for the
// type parameters.

// A concrete type `A`
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
struct Single(A);

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `1T` is generic, it could be anything, including the concrete
// type `A` defined at the top.
struct SingleGen<T>(T);

pub fn generic() {
    // `Single` is concrete and explicitly takes `A`.
    let _s = Single(A);

    // Create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`
    // Here, `SingleGen` has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified:
    let _t = SingleGen(A); // use `A` defined at the top
    let _i32 = SingleGen(6); // uses `i32`
    let _char = SingleGen('a'); // uses `char`

}