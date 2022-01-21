#![feature(
    custom_inner_attributes,
    proc_macro_hygiene,
    stmt_expr_attributes,
    associated_type_defaults
)]


#[you_can::turn_off_the_borrow_checker]
fn main() {
    let mut source = Some(1);
    let mutable_alias = &mut source;
    source = None;
    *mutable_alias = Some(2);
    println!("{source:?}");
    if let Some(ref mut source) = source {
        *source = 3;
    }
    println!("{source:?}");
}
