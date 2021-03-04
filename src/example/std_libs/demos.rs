// Copyright 2021 Ferris Project Authors. License user Apache License.

// Crate method testing

pub fn demo() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668 111111        1";

    let chunked_data = data.split_whitespace();
    let mut collectors = vec![];

    for chunk in chunked_data {
        println!("{}", chunk);
        // Returns an iterator over the [`char`]s of a string slice.
        let total: u32 = chunk
            .chars()
            // `{}` there are optional if the closure body is a single expression.
            .map(|c| c.to_digit(10).expect("should be a digit"))
            .sum();
        collectors.push(total);
    }

    let sum_value: u32 = collectors.iter().sum();
    println!("Sum value is {}", sum_value);

}