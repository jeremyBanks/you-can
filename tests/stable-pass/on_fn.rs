#![warn(unsafe_code)]

#[you_can::turn_off_the_borrow_checker]
fn main() {
    let mut source = 1;
    let mutable_alias = &mut source;
    let immutable_alias = &source;
    source = 2;
    *mutable_alias = 3;
    println!("{immutable_alias}");
}
