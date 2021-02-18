extern crate rand;

use self::rand::random;

#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}

#[derive(Debug)]
enum Book {
    Papery, Electronic
}

pub fn print_struct() {
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("runoob"),
        nation: String::from("China"),
        found: 2013
    };

    // print struct
    println!("{:?}", site);

    // print enum
    let book = Book::Electronic;
    println!("{:?}", book);

    // rand lib
    let x: u8 = random();
    println!("{}", x);

}

