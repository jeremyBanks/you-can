#![feature(stmt_expr_attributes, proc_macro_hygiene)]

pub fn main() {
    let mut owned = vec![1, 32];
    // unsound mutable aliasing
    #[you_can::turn_off_the_borrow_checker]
    let mut_1 = &mut owned[0];
    #[you_can::turn_off_the_borrow_checker]
    let mut_2 = &mut owned[1];

    // use after free
    drop(owned);
    let undefined = *mut_1 + *mut_2;
    println!("{undefined}");
}
