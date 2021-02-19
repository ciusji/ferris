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
    Papery {index: u32},
    Electronic {url: String}
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
    let book = Book::Papery {index: 10001};
    let ebook = Book::Electronic {url: String::from("url...")};
    println!("{:?}", book);
    println!("{:?}", ebook);
    // switch 语法很经典，但在 Rust 中并不支持
    // 很多语言摒弃 switch 的原因是因为 switch 容易存在因忘记添加 break 而产生的串接运行的问题，
    // Java 和 C# 这类语言是通过安全检查杜绝这种情况出现。
    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }

    // Option 枚举类：Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。

    // if let 语法
    let ii = 0;
    match ii {
        0 => println!("zero"),
        _ => println!("none")
    }

    // rand lib
    let x: u8 = random();
    println!("{}", x);

}

