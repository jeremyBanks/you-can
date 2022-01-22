#![feature(proc_macro_hygiene, stmt_expr_attributes)]

pub fn main() {
    let mut source = Some(1);
    let inner_mut = &*source.as_ref().unwrap();
    let mutable_alias = &mut source;

    source = None;
    *mutable_alias = Some(2);

    #[you_can::turn_off_the_borrow_checker]
    {
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
    }

    println!("{source:?}");
}
