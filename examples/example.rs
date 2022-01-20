#![feature(
    custom_inner_attributes,
    proc_macro_hygiene,
    stmt_expr_attributes,
    associated_type_defaults
)]

#[you_can::turn_off_the_borrow_checker]
fn main() {
    let mut a = 1;
    a += 1;
    println!("{a}");
}
