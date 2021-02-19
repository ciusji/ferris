use std::fs;
use std::io::prelude::*;
use std::fs::File;

pub fn read() {
    let path = "/Users/admin/Desktop/xx.txt";

    // // u8 类型集合
    // let content1 = fs::read(path).unwrap();
    // println!("{:?}", content1);

    // file write
    // overwrite();
    create();

    // string
    let content2 = fs::read_to_string(path).unwrap();
    println!("{:?}", content2);

}

fn overwrite() {
    let path = "/Users/admin/Desktop/xx.txt";
    fs::write(path, "123").unwrap();
}

fn create() {
    let path = "/Users/admin/Desktop/xx.txt";
    let mut file = File::create(path).unwrap();
    file.write(b"Nothing").unwrap();
}
