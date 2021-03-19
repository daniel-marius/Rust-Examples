// extern crate testlib;
// use testlib::A::B::*;

mod A;
mod C;

pub mod a {
    pub mod b {
        pub mod c {
            pub mod d {
                pub fn e() {}
            }
        }
    }
}

enum Ex {
    A2,
    B2,
    C2,
}

use a::b::c::d;
use Ex::{ A2, B2 };

fn main() {
    A::b();
    A::B::a();
    C::b();
    d::e();
}
