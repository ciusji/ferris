fn main() {
    // println 不是一个函数，而是一个宏
    println!("Hello, world!");

    println!("Hello {0}, Hahahaha", "Rust");

    // 其他转义同 c 语言，这里仅需要注意 {{ 和 }}
    println!("{{}}");
}
