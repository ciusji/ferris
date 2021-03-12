//! Copyright 2021 Ferris Project Authors. License user Apache License.

// A generic serialization/deserialization framework.

// use serde::{Serialize, Deserialize};
//
// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// pub fn serde_usage() {
//     let point = Point { x: 1, y: 2};
//
//     // Convert the Point to a JSON string.
//     let serialized = serde_json::to_string(&point).unwrap();
//
// }