// mod 类似 import 作用
mod pro1;
mod a;
mod b;

use pro1::print_one;
use a::a::print_a;
use b::b::types;
use b::bb::print_struct;
use b::nation::nation::government::govern;
use b::pai::print_pi;
use b::panic::panic_usage;
use b::generic::max;
use b::lifetime::longer;

// scr 目录下的 main.rs 源文件模式是二进制箱的根，编译之后的二进制箱将于包名相同
fn main() {
    // println 不是一个函数，而是一个宏
    println!("Hello, world!");

    println!("你好 {0} 2021，Rust 发光的一年。", "世界！");

    // 其他转义同 c 语言，这里仅需要注意 {{ 和 }}
    println!("{{}}");

    // rust 不支持 ++ 和 -- 操作，因为这样会减弱开发者对改变变量的意识能力
    // cargo.toml 不写 [[bin]] 标签，默认执行 src/main.rs
    print_one();
    print_a();
    types();
    print_struct();
    govern();
    print_pi();

    let a = [1, 4, 2, 6, 90, 10];
    println!("Max: {}", max(&a));

    let r;
    {
        let s1 = "rust";
        let s2 = "ecmascript";
        r = longer(s1, s2);
        println!("{} is longer", r);
    }

    panic_usage();

}
