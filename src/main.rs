pub mod shared;

use crate::problems::x1;
use crate::problems::x2;
use crate::problems::x3;
use crate::problems::x4;
use crate::problems::x5;
use crate::problems::x6;
use crate::problems::x7;
use crate::problems::x8;
use crate::problems::x9;
use crate::problems::x10;
pub mod problems;

fn main() {

    let problem = 10;
    if problem == 1 {
        x1::p1();
        x1::p2();
    } else if problem == 2 {
        x2::p1();
        x2::p2();
    } else if problem == 3 {
        x3::p1();
        x3::p2();
    } else if problem == 4 {
        x4::p1p2();
    } else if problem == 5 {
        x5::p1();
        x5::p2();
    } else if problem == 6 {
        x6::p1();
    } else if problem == 7 {
        //x7::p1();
        x7::p2();
    } else if problem == 8 {
        x8::p1();
        x8::p2();
    } else if problem == 9 {
        x9::p1();
        x9::p2();
    } else if problem ==10{
    	x10::p1p2();
    }
}
