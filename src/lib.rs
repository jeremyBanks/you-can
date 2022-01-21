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
/// ### Example
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
/// `&mut` operators or the `ref` and `ref mut` bindings, and wraps them with
/// [`::unbounded::reference()`] to [unbind their lifetimes][UBL], causing the
/// borrow checker to effectively ignore them.
///
/// #### Expanded
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
/// illegally composing lifetimes created elsewhere, or references created
/// implicitly. As a workaround, prefixing `&*` can sometimes be used to force
/// an explicit (unbindable) reference where one is needed, such as as in the
/// example below.
///
/// ### Example
///
/// ```
/// #[you_can::turn_off_the_borrow_checker]
/// fn main() {
///     let mut source = Some(1);
///     let inner_mut = &*source.as_ref().unwrap();
///     let mutable_alias = &mut source;
///
///     source = None;
///     *mutable_alias = Some(2);
///
///     if let Some(ref mut inner_a) = source {
///         match source {
///             Some(ref mut inner_b) => {
///                 *inner_b = inner_mut + 1;
///                 *inner_a = inner_mut + 2;
///             },
///             None => {
///                 println!("none");
///             },
///         }
///     }
///
///     println!("{source:?}");
/// }
/// ```
///
/// #### Expanded
///
/// ```
/// fn main() {
///     let mut source = Some(1);
///     let inner_mut = ::unbounded::reference(&*source.as_ref().unwrap());
///     let mutable_alias = ::unbounded::reference(&mut source);
///     source = None;
///     *mutable_alias = Some(2);
///     if let Some(ref mut inner_a) = source {
///         let inner_a = ::unbounded::reference(inner_a);
///         match source {
///             Some(ref mut inner_b) => {
///                 let inner_b = ::unbounded::reference(inner_b);
///                 *inner_b = inner_mut + 1;
///                 *inner_a = inner_mut + 2;
///             },
///             None => {
///                 println!("none");
///             },
///         }
///     }
///     println!("{source:?}");
/// }
/// ```
///
/// [OFF]: https://steveklabnik.com/writing/you-can-t-turn-off-the-borrow-checker-in-rust
/// [REF]: https://doc.rust-lang.org/std/primitive.reference.html
/// [NLL]: https://rust-lang.github.io/rfcs/2094-nll.html
/// [UBL]: https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html
pub use you_can_build_macros::turn_off_the_borrow_checker;
