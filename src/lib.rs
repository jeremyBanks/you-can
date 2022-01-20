//! # \#[you_can::turn_off_the_borrow_checker]
//!
//! [You can't "turn off the borrow checker" in Rust][1], and you shouldn't want
//! to. Rust references aren't pointers, and the compiler is free to decimate
//! code that tries to use references as though they are.
//!
//! However, if you would like to pretend the borrow checker doesn't exist for
//! **educational reasons and never in production code**, this crate provides a
//! macro that will suppress many (though not all) borrow checker errors in the
//! code it's applied to.
//!
//! ```compile_fail
//! fn main() {
//!    let mut source = 1;
//!    let mutable_alias = &mut source;
//!    source = 2; // error: cannot assign to `source` because it is borrowed
//!    *mutable_alias = 3;
//!    println!("{}", source);
//! }
//! ```
//!
//! ```
//! #[you_can::turn_off_the_borrow_checker]
//! fn main() {
//!     let mut source = 1;
//!     let mutable_alias = &mut source;
//!     source = 2;
//!     *mutable_alias = 3;
//!     println!("{}", source);
//! }
//! ```
//!
//! This shouldn't break any otherwise-valid code, but it will allow unsound and
//! unsafe nonsense.
//!
//! This is implemented by finding every reference created in the code by use
//! of the `&` or `&mut` operators, or from bindings with `ref` and `ref mut`,
//! and wrapping them with [`::unbounded::reference()`] to unbind their
//! lifetimes so the borrow checker effectively ignores them.
//!
//! ```
//! fn main() {
//!     let mut source = 1;
//!     let mutable_alias = ::unbounded::reference(&mut source);
//!     source = 2;
//!     *mutable_alias = 3;
//!     println!("{}", source);
//! }
//! ```
//!
//! [1]: https://steveklabnik.com/writing/you-can-t-turn-off-the-borrow-checker-in-rust
#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

#[doc(hidden)]
pub use you_can_build_macros::*;
