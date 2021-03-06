//! Utilities for pinning

#![no_std]
#![warn(missing_docs, missing_debug_implementations)]
#![deny(bare_trait_objects)]
#![allow(unknown_lints)]
#![doc(html_root_url = "https://docs.rs/pin-utils/0.1.0")]

#[macro_use]
mod stack_pin;
#[macro_use]
mod projection;

// Not public API.
#[doc(hidden)]
pub mod __private {
    pub use core::pin::Pin;
}
