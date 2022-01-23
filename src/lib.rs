//! you shouldn't

#![no_std]
#![allow(clippy::needless_doctest_main)]
#![warn(unused_extern_crates)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

#[doc = include_str!("../README.md")]
pub use you_can_build_macros::turn_off_the_borrow_checker;

/// Unbinds the lifetime in a [reference][REF] (`&T` or `&mut T`). This
/// disconnects the input lifetime from the output lifetime, so use of the
/// output reference will no longer cause input reference to be kept "live" (as
/// defined in [RFC 2094: non-lexical lifetimes][NLL]). The output reference is
/// given an [unbounded lifetime][UBL]. This causes the borrow checker to
/// effectively ignore it.
///
/// The output lifetime may still be implicitly bound by any references within
/// the referenced type `T`. You may need to unbind these as well, or replace
/// them with `'static`.
///
/// [REF]: https://doc.rust-lang.org/std/primitive.reference.html
/// [NLL]: https://rust-lang.github.io/rfcs/2094-nll.html
/// [UBL]: https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html
///
/// # Safety
///
/// This is **unsound and unsafe**. Invoking this functions risks incurring the
/// wrath of the optimizer. Rust references are not pointers. For edutainment
/// purposes only.
///
/// # Examples
///
/// Here we create two (aliasing) unbounded mutable references at once, and then
/// continue to use them even after the original value is dropped (and its true
/// lifetime has ended).
///
/// ```
/// let mut x = 0;
///
/// let mut_1 = unsafe { ::you_can::borrow_unchecked(&mut x) };
/// let mut_2 = unsafe { ::you_can::borrow_unchecked(&mut x) };
///
/// drop(x);
///
/// *mut_1 = 1;
/// *mut_2 = 2;
///
/// assert_eq!(*mut_1, *mut_2, "I hope LLVM is in a good mood!");
/// ```
#[inline(always)]
pub unsafe fn borrow_unchecked<
    'original,
    'unbounded,
    Ref: BorrowUnchecked<'original, 'unbounded>,
>(
    reference: Ref,
) -> Ref::Unbounded {
    unsafe { BorrowUnchecked::borrow_unchecked(reference) }
}

/// Used by [`borrow_unchecked`][self::borrow_unchecked] to abstract over `&T`
/// and `&mut T`
///
/// # Safety
///
/// _This section intentionally left blank._
#[doc(hidden)]
pub unsafe trait BorrowUnchecked<'original, 'unbounded> {
    type Unbounded;

    /// # Safety
    ///
    /// _This section intentionally left blank._
    unsafe fn borrow_unchecked(self) -> Self::Unbounded;
}

unsafe impl<'original, 'unbounded, T: 'unbounded> BorrowUnchecked<'original, 'unbounded>
    for &'original T
{
    type Unbounded = &'unbounded T;

    #[inline(always)]
    unsafe fn borrow_unchecked(self) -> Self::Unbounded {
        unsafe { ::core::mem::transmute(self) }
    }
}

unsafe impl<'original, 'unbounded, T: 'unbounded> BorrowUnchecked<'original, 'unbounded>
    for &'original mut T
{
    type Unbounded = &'unbounded mut T;

    #[inline(always)]
    unsafe fn borrow_unchecked(self) -> Self::Unbounded {
        unsafe { ::core::mem::transmute(self) }
    }
}
