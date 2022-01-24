//! [`#[you_can::turn_off_the_borrow_checker]`][turn_off_the_borrow_checker]
#![no_std]
#![allow(clippy::needless_doctest_main)]
#![warn(unused_extern_crates)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

#[doc = include_str!("./turn_off_the_borrow_checker.md")]
pub use you_can_build_macros::turn_off_the_borrow_checker;

#[doc = include_str!("./borrow_unchecked.md")]
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

#[doc(hidden)]
pub unsafe trait BorrowUnchecked<'original, 'unbounded> {
    type Unbounded;

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
