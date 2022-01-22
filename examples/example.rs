#![feature(custom_inner_attributes, proc_macro_hygiene, stmt_expr_attributes)]

mod inner {
    #![you_can::turn_off_the_borrow_checker]
    pub fn main() {
        let mut source = Some(1);
        let inner_mut = &*source.as_ref().unwrap();
        let mutable_alias = &mut source;

        source = None;
        *mutable_alias = Some(2);

        if let Some(ref mut inner_a) = source {
            match source {
                Some(ref mut inner_b) => {
                    *inner_b = inner_mut + 1;
                    *inner_a = inner_mut + 2;
                },
                None => {
                    println!("none");
                },
            }
        }

        println!("{source:?}");
    }
}

fn main() {
    inner::main();

    let x = Some(1);

    x = #[you_can::turn_off_the_borrow_checker]
    {
        let y = &*x.as_ref().unwrap();
        let z = &*x.as_ref().unwrap();
    };
}
