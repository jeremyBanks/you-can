//! [`#[you_can::turn_off_the_borrow_checker]`][turn_off_the_borrow_checker]
#![no_std]
#![allow(clippy::needless_doctest_main)]
#![warn(unused_extern_crates)]
#![deny(unsafe_op_in_unsafe_fn)]
#[doc = include_str!("../README.md")]
pub use you_can_build_macros::turn_off_the_borrow_checker;
