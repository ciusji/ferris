// 所有的系统库模块都是被默认导入的，所以在使用的时候只需要使用 use 关键字简化路径就可以方便使用了。
use std::f64::consts::PI;

pub fn print_pi() {
    println!("{}", (PI / 2.0).sin());
}