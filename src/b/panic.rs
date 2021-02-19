// 错误处理
// Rust 有一套独特的处理异常的机制，并不像其他语言中的 try 机制那样简单。
// 首先，程序中一般会出现两种错误：可恢复错误和不可恢复错误。
// 大多数编程语言不区分这两种错误，并用 Exception 类来标识错误。在 Rust 中没有 Exception。
// 对于可恢复错误用 Result<T, E> 类来处理；对于不可恢复错误用 panic! 宏来处理。

use std::fs::File;

pub fn panic_usage() {
    error_usage();
    panic!("error occurred");
    println!("Hello, Rust!");
}

fn error_usage() {
    // 如果想使一个可恢复错误处理按照不可恢复错误处理，Result 提供了两种方法：unwrap() 和 except(message: &str)
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").except("Failed to open");
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.");
        }
    }
}