#![cfg_attr(not(test), no_std)]

pub use do_you_yield_macro::gn;

#[doc(hidden)]
pub mod sync;

#[doc(hidden)]
#[cfg(feature = "async")]
pub mod not_sync;

mod waker;
