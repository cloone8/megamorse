//! Megamorse core library.
//!
//! This library contains the core types used by the Megamorse library.
//! There's no need to use this library directly, as it is re-exported by
//! the main Megamorse library.
#![no_std]

mod code;
mod sequence;
mod word;

#[doc(inline)]
pub use code::*;
#[doc(inline)]
pub use sequence::*;
#[doc(inline)]
pub use word::*;
