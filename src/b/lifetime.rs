// Rust 生命周期
// Rust 生命周期机制是所有权机制同等重要的资源管理机制。
// 之所以引入这个概念主要是应对复杂类型系统中资源管理的问题。

// 引用必须在值的生命周期内才有效。

// 生命周期注解：描述引用声明周期的办法。虽然这样不能改变引用的生命周期，但可以在合适的地方声明两个引用的声明周期一致。
/*
 * &i32 // 常规引用
 * &'a i32 // 含有声明周期注释的引用
 * &'a mut i32 // 可变型含有生命周期注释的引用
 */

// 'a 表示一个声明周期 | 声明周期注解示例
pub fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}