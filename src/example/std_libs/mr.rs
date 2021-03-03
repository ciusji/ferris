// Copyright 2021 Ferris Project Authors. License user Apache License.

use std::thread;

pub fn mr_task() {
    // This is our data to process.
    // We will calculate the sum of all digits via threaded map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    // **Map** phase
    // Divide our data into segments, and apply initial processing
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread, which we MUST keep to access the returned value
        //
        // `move || -> u32` is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variable ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from the closure itself so we could have
        // left that out
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of his segment:
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, result);
            result
        }))
    }

    // **Reduce** phase
    // Collect our intermediate result, and combine them into a final result
    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's return-value
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // Combine all intermediate sums into a single final sum.
    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result);

}