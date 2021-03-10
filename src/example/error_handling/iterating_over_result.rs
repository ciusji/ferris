//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Iterating over `Result`s

pub fn iterating_over_result() {
    let strings = vec!["tofu", "93", "18"];

    // Item::map operation might fail
    // let numbers: Vec<_> = strings.into_iter()
    //     .map(|s| s.parse::<i32>())
    //     .collect();

    // filter_map call a function and filters out the results that are None.
    let numbers: Vec<_> = strings.into_iter()
        .flat_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("Result {:?}", numbers);

}