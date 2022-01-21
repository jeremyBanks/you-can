#![no_std]
#![allow(clippy::needless_doctest_main)]
#![warn(unused_extern_crates)]
#![deny(unsafe_op_in_unsafe_fn)]
/// [You can't "turn off the borrow checker" in Rust][OFF], and you shouldn't
/// want to. Rust references aren't pointers, and the compiler is free to
/// decimate code that tries to use references as though they are. However, if
/// you would like to pretend the borrow checker doesn't exist **for educational
/// purposes and never in production code**, this macro that will suppress many
/// (though not all) borrow checker errors in the code it's applied to.
///
/// ```compile_fail
/// fn main() {
///    let mut source = 1;
///    let mutable_alias = &mut source;
///    source = 2; // error: cannot assign to `source` because it is borrowed
///    *mutable_alias = 3;
///    println!("{source}");
/// }
/// ```
///
/// ```
/// #[you_can::turn_off_the_borrow_checker]
/// fn main() {
///     let mut source = 1;
///     let mutable_alias = &mut source;
///     source = 2;
///     *mutable_alias = 3;
///     println!("{source}");
/// }
/// ```
///
/// This shouldn't break any otherwise-valid code, but it will allow unsound and
/// unsafe nonsense.
///
/// ## Explanation
///
/// The macro looks for references created in the code by use of the `&` or
/// `&mut` operators, or from bindings with `ref` and `ref mut`, and wraps
/// them with [`::unbounded::reference()`] to [unbind their lifetimes][UBL],
/// causing the borrow checker to effectively ignore them.
///
/// ```
/// fn main() {
///     let mut source = 1;
///     let mutable_alias = ::unbounded::reference(&mut source);
///     source = 2;
///     *mutable_alias = 3;
///     println!("{source}");
/// }
/// ```
///
/// This approached is limited. It can't suppress errors resulting from the code
/// illegally composing lifetimes created elsewhere.
///
/// [OFF]: https://steveklabnik.com/writing/you-can-t-turn-off-the-borrow-checker-in-rust
/// [REF]: https://doc.rust-lang.org/std/primitive.reference.html
/// [NLL]: https://rust-lang.github.io/rfcs/2094-nll.html
/// [UBL]: https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html
pub use you_can_build_macros::turn_off_the_borrow_checker;
