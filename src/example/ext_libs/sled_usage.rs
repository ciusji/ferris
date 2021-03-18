//! Copyright 2021 Ferris Project Authors. License user Apache License.

// sled usage

use sled;

pub fn store() {
    let db = sled::open("/Users/admin/Tabletrix/db").unwrap();

    // let mut batch = sled::Batch::default();
    //
    // for i in 0..1_000_000 {
    //     // batch.insert(i as String::from(), i);
    // }
    //
    // db.apply_batch(batch).unwrap();

    println!("tree value: {}", db.len());
}