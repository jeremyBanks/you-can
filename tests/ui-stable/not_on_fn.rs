// compile-fail
#![warn(unsafe_code)]

fn main() {
    let mut source = 1;
    let mutable_alias = &mut source;
    source = 2;
    *mutable_alias = 3;
    println!("{source}");
}
