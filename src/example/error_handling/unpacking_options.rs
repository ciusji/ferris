//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Unpacking options with `?`
// You can unpack options by using match statements, but it's often easier to use the ? operator.
#![allow(dead_code)]

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}
#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // Gets the area code of the phone number of the person's job, if it exists.
    fn work_phone_area_code(&self) -> Option<u8> {
        // self.job.unwrap().phone_number.unwrap().area_code
        self.job?.phone_number?.area_code
    }
}


pub fn unpacking_options() {
    let p = Person{
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 43992222,
            })
        })
    };

    // ? can only be only used in a function that returns Result or Option.
    let current_number = p.work_phone_area_code().unwrap();
    println!("Current number: {}", current_number);

}