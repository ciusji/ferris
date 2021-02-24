// Copyright 2021 Ferris Project Authors. License user Apache License.

use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    // `f` is a buffer and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like 'format!', but it will write the formatted string into a buffer
        write!(f, "{:} {:.3}'{} {:.3}'{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn formatting() {
    for city in [
        City { name: "Dublin", lat: 53.3477778, lon: -6.259772 },
        City { name: "Oslo", lat: 59.559, lon: 10.75},

    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color);
    }
}