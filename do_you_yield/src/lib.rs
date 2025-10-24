#![cfg_attr(not(test), no_std)]

pub use do_you_yield_macro::gn;

#[doc(hidden)]
pub mod sync;

mod waker;
