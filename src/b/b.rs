pub fn types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z);

    let a = [1, 2, 3, 4, 5, 6];
    // 如何打印整个数组
    println!("{}", a[0]);
    // 打印整个数组
    println!("{:?}", a);

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
    // 所有权的概念是为了让 Rust 在编译阶段解决更有效地分析内存资源的有用性以实现内存管理而诞生的概念。
    // 所有权的三条基本规则：
    // - Rust 中的每个值都有一个变量，称为其所有者；
    // 一次只能有一个所有者；
    // 当所有所有者不再程序运行范围时，该值将被删除；

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1={}, s2={}", s1, s2);

    // "引用" 是变量的间接访问方式，& 运算符可以取变量的引用
    let s3 = &s1;
    println!("s1={}, s3={}", s1, s3);
    println!("s1 len={}", s1.len());

    // "切片" 是对数据值的部分引用
    let s4 = &s1[..5];
    println!("{}", s4);
    let s5 = &s[..];
    println!("{:?}", s5);


}