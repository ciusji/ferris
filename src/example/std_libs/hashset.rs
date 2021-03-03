// Copyright 2021 Ferris Project Authors. License user Apache License.

// HashSet
// Consider a HashSet as a s HashMap where we just care about the keys (HashSet<T> is, actually,
// just a wrapper around HashMap<T, ()>).
// A HashSet's unique feature is that is guaranteed to not have duplicate element. That's the
// contract that any set collection fulfills. HashSet is just one implementation.
// But sets can do more than that.
// Sets have 4 primary operations (all of the following calls return an iterator):
// - union: get all the unique elements in both sets.
// - difference: get all the elements that are in the first set but not in the second.
// - intersection: get all the elements that are only in both sets.
// - symmetric_difference: get all the element that are in on set or the other, bu not both.

use std::collections::HashSet;

pub fn hash_set() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` return false if there was a value already present.
    assert!(!b.insert(4), "Value 4 is already in set B");

    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("Union: {:?}", a.union(&b));
    println!("Difference: {:?}", a.difference(&b));
    println!("Intersection: {:?}", a.intersection(&b));
    println!("Symmetric Difference: {:?}", a.symmetric_difference(&b));

}