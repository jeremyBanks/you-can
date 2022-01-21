//! **Unsound and unsafe**! For edutainment purposes only.
#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

/// Unbinds the lifetime in a [reference][REF] (`&T` or `&mut T`). This
/// disconnects the input lifetime from the output lifetime, so use of the
/// output reference will no longer cause input reference to be kept "live" (as
/// defined in [RFC 2094: non-lexical lifetimes][NLL]). The output reference is
/// given an [unbounded lifetime][UBL], so the borrow checker is free to coerce
/// it to whatever is expected.
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
/// This is **unsound and unsafe**, even though it is marked as a safe function!
/// Invoking this functions risks incurring the wrath of the optimizer. Rust
/// references are not pointers. For edutainment purposes only.
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
/// let mut_1 = unsafe { ::unbounded::reference(&mut x) };
/// let mut_2 = unsafe { ::unbounded::reference(&mut x) };
///
/// drop(x);
///
/// *mut_1 = 1;
/// *mut_2 = 2;
///
/// assert_eq!(*mut_1, *mut_2, "I hope LLVM is in a good mood!");
/// ```
#[inline(always)]
pub unsafe fn reference<'original, 'unbounded, Ref: Reference<'original, 'unbounded>>(
    reference: Ref,
) -> Ref::Unbounded {
    unsafe { reference.unbounded() }
}

/// Used by [`reference`][self::reference] to abstract over `&T` and `&mut T`
///
/// # Safety
///
/// _This section intentionally left blank._
pub unsafe trait Reference<'original, 'unbounded> {
    type Unbounded;

    /// # Safety
    ///
    /// _This section intentionally left blank._
    unsafe fn unbounded(self) -> Self::Unbounded;
}

unsafe impl<'original, 'unbounded, T: 'unbounded> Reference<'original, 'unbounded>
    for &'original T
{
    type Unbounded = &'unbounded T;

    #[inline(always)]
    unsafe fn unbounded(self) -> Self::Unbounded {
        unsafe { ::core::mem::transmute(self) }
    }
}

unsafe impl<'original, 'unbounded, T: 'unbounded> Reference<'original, 'unbounded>
    for &'original mut T
{
    type Unbounded = &'unbounded mut T;

    #[inline(always)]
    unsafe fn unbounded(self) -> Self::Unbounded {
        unsafe { ::core::mem::transmute(self) }
    }
}
