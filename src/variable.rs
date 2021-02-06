// mod 模块
mod variable {
    pub mod xx {
        pub fn x() {
            println!(1);
        }
    }
}

// :: 路径分隔符
use crate::variable::xx::x;

// fn 函数
fn main() {
    x()
}