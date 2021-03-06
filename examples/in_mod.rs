#![feature(custom_inner_attributes)]

mod inner {
    #![you_can::turn_off_the_borrow_checker]

    pub fn main() {
        let mut owned = vec![1, 32];

        // unsound mutable aliasing
        let mut_1 = &mut owned[0];
        let mut_2 = &mut owned[1];

        // use after free
        drop(owned);
        let undefined = *mut_1 + *mut_2;
        println!("{undefined}");
    }
}

fn main() {
    inner::main();
}
