// Copyright 2021 Ferris Project Authors. License user Apache License.

// HOF
// Rust provides Higher Order Functions (HOF).

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn higher_order_functions() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;

    for n in 0.. {
        // Square the number
        let n_squared =  n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32
        = (0..).map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);
    println!("functional style: {}", sum_of_squared_odd_numbers);

}