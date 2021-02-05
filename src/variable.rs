mod variable {
    pub mod xx {
        pub fn x() {
            println!(1);
        }
    }
}

use crate::variable::xx::x;

fn main() {
    x()
}