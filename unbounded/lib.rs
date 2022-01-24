//! deprecated on arrival
#![no_std]

/// moved to [`::you_can::borrow_unchecked`]
#[deprecated = "moved to `::you_can::borrow_unchecked`"]
pub unsafe fn reference<
    'original,
    'unbounded,
    Ref: ::you_can::BorrowUnchecked<'original, 'unbounded>,
>(
    reference: Ref,
) -> Ref::Unbounded {
    unsafe { ::you_can::borrow_unchecked(reference) }
}
