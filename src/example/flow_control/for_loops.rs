// Copyright 2021 Ferris Project Authors. License user Apache License.

// For
// The for in construct is able to interact with an Iterator in several ways. As discussed
// in hte section on the iterator trait, by default the for loop will apply the into_iter function
// to the collection. However, this is not the only means of converting collections into iterators.

// `init_iter`, `iter` and `iter_mut` all handle the conversion of a collection into an iterator in
// different ways, by providing different views on the data within.

pub fn for_loops() {
    // `n` will take the values: 1, 2, 3, ..., 100 in each iteration
    for _n in 1..101 {}
    // error: cannot find value `n` in this scope
    // println!("last value n={}", n);


    let names = vec!["Bob", "Frank", "Ferris"];
    // By default, the for loop will apply the into_iter function.
    // - iter: This borrows each element of the collection through each iteration. Thus leaning
    // the collection untouched and available for reuse the loop.
    // - into_iter: This consumes the collection so that on each iteration the exact data is
    // provided. Once the collection has been consumed it is not longer available for reuse as
    // it has been 'moved' within the loop.
    // - iter_mut: This mutably borrows each element of the collection, allowing for the collection
    // to be modified in place.
    for name in names.into_iter() {
        println!("{:?}", name);
    }

}