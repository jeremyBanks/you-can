fn main() {
    let mut source = 1;
    let mutable_alias = &mut source;
    source = 2;
    //~^ ERROR cannot assign to `source` because it is borrowed
    *mutable_alias = 3;
    println!("{source}");
}
