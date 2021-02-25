// Copyright 2021 Ferris Project Authors. License user Apache License.

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, i32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn structures() {
    // Create structure with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age};

    // Print struct
    println!("{:?}", peter);

    // Instantiate a `Point` & access the fields of the point
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: {}, {}", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point coordinates: {}, {}", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    // let Point { x: top_edge, y: left_edge } = point;
    // println!("{}, {}", top_edge, left_edge);

    let _unit = Unit;

    let pair = Pair(0, 1);
    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1)
}