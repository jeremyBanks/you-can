fn main() {
    let mut owned = vec![1, 32];

    // unsound mutable aliasing
    let mut_1 = &mut owned[0];
    let mut_2 = &mut owned[1];
    //~^ ERROR cannot borrow `owned` as mutable more than once at a time

    // use after free
    drop(owned);
    //~^ ERROR cannot move out of `owned` because it is borrowed
    let undefined = *mut_1 + *mut_2;
    println!("{undefined}");
}
