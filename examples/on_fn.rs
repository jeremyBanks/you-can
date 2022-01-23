#![allow(unused_variables)]
#![warn(unsafe_code)]

#[you_can::turn_off_the_borrow_checker]
pub fn main() {
    let mut source = 1;
    let alias_1 = &mut source;
    let alias_2 = &mut source;
}
