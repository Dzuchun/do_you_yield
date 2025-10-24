#![cfg_attr(not(test), no_std)]

pub use yaag_proc_macro::gn;

#[doc(hidden)]
pub mod sync;

#[doc(hidden)]
#[cfg(feature = "async")]
pub mod not_sync;

mod waker;
