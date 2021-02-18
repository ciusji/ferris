fn main() {
    // println 不是一个函数，而是一个宏
    println!("Hello, world!");

    println!("你好 {0} 2021，Rust 发光的一年。", "世界！");

    // 其他转义同 c 语言，这里仅需要注意 {{ 和 }}
    println!("{{}}");
}
