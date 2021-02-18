pub fn types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z);

    let a = [1, 2, 3, 4, 5, 6];
    // 如何打印整个数组
    println!("{}", a[0]);

    // while 循环
    let mut number = 1;
    while number <= 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");

    // loop 循环（Rust 原生的无限循环结构）
    let s = ["R", "U", "N", "O", "O", "B"];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == "O" {
            break;
        }
        println!("{}", ch);
        i += 1;
    }

    // 所有权，对大多数开发者而言是一个比较新颖的概念，它是 Rust 语言为高效实用内存而设计的语法机制。
    // 所有权的概念是为了让 Rust 在编译解决更有效地分析内存资源的有用性以实现内存管理而诞生的概念。




}